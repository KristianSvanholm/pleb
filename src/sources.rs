#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use std::{
  marker::{PhantomData, PhantomPinned},
  mem::MaybeUninit,
  ptr::null,
};

use core_foundation::{
  array::{CFArrayGetCount, CFArrayGetValueAtIndex, CFArrayRef},
  base::{kCFAllocatorDefault, kCFAllocatorNull, CFRelease, CFTypeRef},
  dictionary::{
    CFDictionaryCreateMutableCopy, CFDictionaryGetCount, CFDictionaryGetValue, CFDictionaryRef,
    CFMutableDictionaryRef,
  },
  number::{kCFNumberSInt32Type, CFNumberCreate, CFNumberRef},
  string::{kCFStringEncodingUTF8, CFStringCreateWithBytesNoCopy, CFStringGetCString, CFStringRef},
};

pub type WithError<T> = Result<T, Box<dyn std::error::Error>>;
pub type CVoidRef = *const std::ffi::c_void;

// MARK: CFUtils

pub fn cfnum(val: i32) -> CFNumberRef {
  unsafe { CFNumberCreate(kCFAllocatorDefault, kCFNumberSInt32Type, &val as *const i32 as _) }
}

pub fn cfstr(val: &str) -> CFStringRef {
  // this creates broken objects if string len > 9
  // CFString::from_static_string(val).as_concrete_TypeRef()
  // CFString::new(val).as_concrete_TypeRef()

  unsafe {
    CFStringCreateWithBytesNoCopy(
      kCFAllocatorDefault,
      val.as_ptr(),
      val.len() as isize,
      kCFStringEncodingUTF8,
      0,
      kCFAllocatorNull,
    )
  }
}

pub fn from_cfstr(val: CFStringRef) -> String {
  unsafe {
    let mut buf = Vec::with_capacity(128);
    if CFStringGetCString(val, buf.as_mut_ptr(), 128, kCFStringEncodingUTF8) == 0 {
      panic!("Failed to convert CFString to CString");
    }
    std::ffi::CStr::from_ptr(buf.as_ptr()).to_string_lossy().to_string()
  }
}

pub fn cfdict_get_val(dict: CFDictionaryRef, key: &str) -> Option<CFTypeRef> {
  unsafe {
    let key = cfstr(key);
    let val = CFDictionaryGetValue(dict, key as _);
    CFRelease(key as _);

    match val {
      _ if val.is_null() => None,
      _ => Some(val),
    }
  }
}

// MARK: IOReport Bindings

#[link(name = "IOKit", kind = "framework")]
#[rustfmt::skip]
extern "C" {
  fn IOIteratorNext(iterator: u32) -> u32;
  fn IORegistryEntryGetName(entry: u32, name: *mut i8) -> i32;
  fn IOObjectRelease(obj: u32) -> u32;
}

#[repr(C)]
struct IOReportSubscription {
  _data: [u8; 0],
  _phantom: PhantomData<(*mut u8, PhantomPinned)>,
}

type IOReportSubscriptionRef = *const IOReportSubscription;

#[link(name = "IOReport", kind = "dylib")]
#[rustfmt::skip]
extern "C" {
  fn IOReportCopyAllChannels(a: u64, b: u64) -> CFDictionaryRef;
  fn IOReportCopyChannelsInGroup(a: CFStringRef, b: CFStringRef, c: u64, d: u64, e: u64) -> CFDictionaryRef;
  fn IOReportMergeChannels(a: CFDictionaryRef, b: CFDictionaryRef, nil: CFTypeRef);
  fn IOReportCreateSubscription(a: CVoidRef, b: CFMutableDictionaryRef, c: *mut CFMutableDictionaryRef, d: u64, b: CFTypeRef) -> IOReportSubscriptionRef;
  fn IOReportCreateSamples(a: IOReportSubscriptionRef, b: CFMutableDictionaryRef, c: CFTypeRef) -> CFDictionaryRef;
  fn IOReportCreateSamplesDelta(a: CFDictionaryRef, b: CFDictionaryRef, c: CFTypeRef) -> CFDictionaryRef;
  fn IOReportChannelGetGroup(a: CFDictionaryRef) -> CFStringRef;
  fn IOReportSimpleGetIntegerValue(a: CFDictionaryRef, b: i32) -> i64;
  fn IOReportChannelGetUnitLabel(a: CFDictionaryRef) -> CFStringRef;
}

// MARK: IOReport helpers

fn cfio_get_group(item: CFDictionaryRef) -> String {
  match unsafe { IOReportChannelGetGroup(item) } {
    x if x.is_null() => String::new(),
    x => from_cfstr(x),
  }
}

pub fn cfio_watts(item: CFDictionaryRef, unit: &String) -> WithError<f32> {
  let val = unsafe { IOReportSimpleGetIntegerValue(item, 0) } as f32;
  match unit.as_str() {
    "mJ" => Ok(val / 1e3f32),
    "uJ" => Ok(val / 1e6f32),
    "nJ" => Ok(val / 1e9f32),
    _ => Err(format!("Invalid energy unit: {}", unit).into()),
  }
}

// MARK: IOServiceIterator

pub struct IOServiceIterator {
  existing: u32,
}

impl Drop for IOServiceIterator {
  fn drop(&mut self) {
    unsafe {
      IOObjectRelease(self.existing);
    }
  }
}

impl Iterator for IOServiceIterator {
  type Item = (u32, String);

  fn next(&mut self) -> Option<Self::Item> {
    let next = unsafe { IOIteratorNext(self.existing) };
    if next == 0 {
      return None;
    }

    let mut name = [0; 128]; // 128 defined in apple docs
    if unsafe { IORegistryEntryGetName(next, name.as_mut_ptr()) } != 0 {
      return None;
    }

    let name = unsafe { std::ffi::CStr::from_ptr(name.as_ptr()) };
    let name = name.to_string_lossy().to_string();
    Some((next, name))
  }
}

// MARK: IOReportIterator

pub struct IOReportIterator {
  sample: CFDictionaryRef,
  index: isize,
  items: CFArrayRef,
  items_size: isize,
}

impl IOReportIterator {
  pub fn new(data: CFDictionaryRef) -> Self {
    let items = cfdict_get_val(data, "IOReportChannels").unwrap() as CFArrayRef;
    let items_size = unsafe { CFArrayGetCount(items) } as isize;
    Self { sample: data, items, items_size, index: 0 }
  }
}

impl Drop for IOReportIterator {
  fn drop(&mut self) {
    unsafe {
      CFRelease(self.sample as _);
    }
  }
}

#[derive(Debug)]
pub struct IOReportIteratorItem {
  pub group: String,
  pub unit: String,
  pub item: CFDictionaryRef,
}

impl Iterator for IOReportIterator {
  type Item = IOReportIteratorItem;

  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.items_size {
      return None;
    }

    let item = unsafe { CFArrayGetValueAtIndex(self.items, self.index) } as CFDictionaryRef;

    let group = cfio_get_group(item);
    let unit = from_cfstr(unsafe { IOReportChannelGetUnitLabel(item) }).trim().to_string();

    self.index += 1;
    Some(IOReportIteratorItem { group, unit, item })
  }
}

// MARK: IOReport

unsafe fn cfio_get_chan(items: Vec<(&str, Option<&str>)>) -> WithError<CFMutableDictionaryRef> {
  // if no items are provided, return all channels
  if items.len() == 0 {
    let c = IOReportCopyAllChannels(0, 0);
    let r = CFDictionaryCreateMutableCopy(kCFAllocatorDefault, CFDictionaryGetCount(c), c);
    CFRelease(c as _);
    return Ok(r);
  }

  let mut channels = vec![];
  for (group, subgroup) in items {
    let gname = cfstr(group);
    let sname = subgroup.map_or(null(), |x| cfstr(x));
    let chan = IOReportCopyChannelsInGroup(gname, sname, 0, 0, 0);
    channels.push(chan);

    CFRelease(gname as _);
    if subgroup.is_some() {
      CFRelease(sname as _);
    }
  }

  let chan = channels[0];
  for i in 1..channels.len() {
    IOReportMergeChannels(chan, channels[i], null());
  }

  let size = CFDictionaryGetCount(chan);
  let chan = CFDictionaryCreateMutableCopy(kCFAllocatorDefault, size, chan);

  for i in 0..channels.len() {
    CFRelease(channels[i] as _);
  }

  if cfdict_get_val(chan, "IOReportChannels").is_none() {
    return Err("Failed to get channels".into());
  }

  Ok(chan)
}

unsafe fn cfio_get_subs(chan: CFMutableDictionaryRef) -> WithError<IOReportSubscriptionRef> {
  let mut s: MaybeUninit<CFMutableDictionaryRef> = MaybeUninit::uninit();
  let rs = IOReportCreateSubscription(std::ptr::null(), chan, s.as_mut_ptr(), 0, std::ptr::null());
  if rs == std::ptr::null() {
    return Err("Failed to create subscription".into());
  }

  s.assume_init();
  Ok(rs)
}

pub struct IOReport {
  subs: IOReportSubscriptionRef,
  chan: CFMutableDictionaryRef,
}

impl IOReport {
  pub fn new(channels: Vec<(&str, Option<&str>)>) -> WithError<Self> {
    let chan = unsafe { cfio_get_chan(channels)? };
    let subs = unsafe { cfio_get_subs(chan)? };

    Ok(Self { subs, chan })
  }

  pub fn get_sample(&self, duration: u64) -> IOReportIterator {
    unsafe {
      let sample1 = IOReportCreateSamples(self.subs, self.chan, null());
      std::thread::sleep(std::time::Duration::from_millis(duration));
      let sample2 = IOReportCreateSamples(self.subs, self.chan, null());

      let sample3 = IOReportCreateSamplesDelta(sample1, sample2, null());
      CFRelease(sample1 as _);
      CFRelease(sample2 as _);
      IOReportIterator::new(sample3)
    }
  }
}

impl Drop for IOReport {
  fn drop(&mut self) {
    unsafe {
      CFRelease(self.chan as _);
      CFRelease(self.subs as _);
    }
  }
}

// MARK: IOHIDSensors

pub struct IOHIDSensors {
  sensors: CFDictionaryRef,
}

impl Drop for IOHIDSensors {
  fn drop(&mut self) {
    unsafe {
      CFRelease(self.sensors as _);
    }
  }
}
