// Copyright 2013 The Servo Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/* automatically generated by rust-bindgen */

use std::libc::*;

type enum_unnamed1 = c_uint;
static STBI_default: u32 = 0_u32;
static STBI_grey: u32 = 1_u32;
static STBI_grey_alpha: u32 = 2_u32;
static STBI_rgb: u32 = 3_u32;
static STBI_rgb_alpha: u32 = 4_u32;

#[link_args="-L. -lstb-image"]
#[nolink]
extern {
}

pub mod bindgen {
  use std::libc::*;

  type stbi_uc = c_uchar;
  struct stbi_io_callbacks {
    read: *u8,
    skip: *u8,
    eof: *u8,
  }

  pub extern {

fn stbi_load_from_memory(buffer: *stbi_uc, len: c_int, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *stbi_uc;

fn stbi_load(filename: *c_char, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *stbi_uc;

fn stbi_load_from_file(f: *FILE, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *stbi_uc;

fn stbi_load_from_callbacks(clbk: *stbi_io_callbacks, user: *c_void, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *stbi_uc;

fn stbi_loadf_from_memory(buffer: *stbi_uc, len: c_int, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *c_float;

fn stbi_loadf(filename: *c_char, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *c_float;

fn stbi_loadf_from_file(f: *FILE, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *c_float;

fn stbi_loadf_from_callbacks(clbk: *stbi_io_callbacks, user: *c_void, x: *mut c_int, y: *mut c_int, comp: *mut c_int, req_comp: c_int) -> *c_float;

fn stbi_hdr_to_ldr_gamma(gamma: c_float);

fn stbi_hdr_to_ldr_scale(scale: c_float);

fn stbi_ldr_to_hdr_gamma(gamma: c_float);

fn stbi_ldr_to_hdr_scale(scale: c_float);

fn stbi_is_hdr_from_callbacks(clbk: *stbi_io_callbacks, user: *c_void) -> c_int;

fn stbi_is_hdr_from_memory(buffer: *stbi_uc, len: c_int) -> c_int;

fn stbi_is_hdr(filename: *c_char) -> c_int;

fn stbi_is_hdr_from_file(f: *FILE) -> c_int;

fn stbi_failure_reason() -> *c_char;

fn stbi_image_free(retval_from_stbi_load: *c_void);

fn stbi_info_from_memory(buffer: *stbi_uc, len: c_int, x: *c_int, y: *c_int, comp: *c_int) -> c_int;

fn stbi_info_from_callbacks(clbk: *stbi_io_callbacks, user: *c_void, x: *c_int, y: *c_int, comp: *c_int) -> c_int;

fn stbi_info(filename: *c_char, x: *c_int, y: *c_int, comp: *c_int) -> c_int;

fn stbi_info_from_file(f: *FILE, x: *c_int, y: *c_int, comp: *c_int) -> c_int;

fn stbi_set_unpremultiply_on_load(flag_true_if_should_unpremultiply: c_int);

fn stbi_convert_iphone_png_to_rgb(flag_true_if_should_convert: c_int);

fn stbi_zlib_decode_malloc_guesssize(buffer: *c_char, len: c_int, initial_size: c_int, outlen: *c_int) -> *c_char;

fn stbi_zlib_decode_malloc(buffer: *c_char, len: c_int, outlen: *c_int) -> *c_char;

fn stbi_zlib_decode_buffer(obuffer: *c_char, olen: c_int, ibuffer: *c_char, ilen: c_int) -> c_int;

fn stbi_zlib_decode_noheader_malloc(buffer: *c_char, len: c_int, outlen: *c_int) -> *c_char;

fn stbi_zlib_decode_noheader_buffer(obuffer: *c_char, olen: c_int, ibuffer: *c_char, ilen: c_int) -> c_int;

  }
}
