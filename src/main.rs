use anyhow::{Ok, Result, anyhow, bail};
use clap::Parser;
use core::convert::Into;
use core::{clone::Clone, iter::Iterator, todo};
use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::ToPrimitive;
use std::io::{self, Write};
use std::process::exit;
use tracing::trace;
use tracing_subscriber::EnvFilter;
use wasmtime::*;
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    filename: String,
}

#[derive(Default, Debug)]
struct HootStatus {
    _count: u32,
}

impl HootStatus {
    fn new(count: u32) -> Self {
        Self { _count: count }
    }
}

#[derive(Default, Debug)]
struct WeakMap {
    _count: u32,
}

impl WeakMap {
    fn new(count: u32) -> Self {
        Self { _count: count }
    }
}
fn bool_to_i32(b: bool) -> i32 {
    if b { 1 } else { 0 }
}

struct Runtime {
    engine: Engine,
}

impl Runtime {
    fn new(config: Option<Config>) -> Result<Self> {
        let mut config: Config = match config {
            Some(x) => x,
            None => Config::default(),
        };
        config.wasm_gc(true);
        config.wasm_tail_call(true);
        config.wasm_backtrace(true);
        config.wasm_function_references(true);
        config.wasm_reference_types(true);
        config.wasm_gc(true);
        config.wasm_tail_call(true);
        config.wasm_exceptions(true);
        config.wasm_stack_switching(true);
        let engine = Engine::new(&config)?;
        Ok(Self { engine })
    }
}

fn extern_ref_to_bigint(c: Rooted<ExternRef>, caller: &Caller<'_, HootStatus>) -> Result<BigInt> {
    let cc = c.data(&caller)?.expect("wtf");
    let nu = cc
        .downcast_ref::<BigInt>()
        .ok_or_else(|| anyhow::anyhow!("externref is not bignum!"))?;
    Ok(nu.clone())
}
fn extern_ref_to_string(c: Rooted<ExternRef>, caller: &Caller<'_, HootStatus>) -> Result<String> {
    let cc = c.data(&caller)?.expect("wtf");
    let nu = cc
        .downcast_ref::<String>()
        .ok_or_else(|| anyhow::anyhow!("externref is not string!"))?;
    Ok(nu.clone())
}

fn io_read_file(_caller: Caller<'_, HootStatus>, _a: Rooted<ExternRef>, _b: i32) -> Result<i32> {
    trace!("read_file");
    todo!()
}

fn io_close_file(_caller: Caller<'_, HootStatus>, _a: Rooted<ExternRef>) -> Result<()> {
    trace!("close_file");
    todo!()
}
fn io_seek_file(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: i32,
    _c: i32,
) -> Result<i32> {
    trace!("seek_file");
    todo!()
}

fn io_read_stdin(caller: Caller<'_, HootStatus>) -> Result<Rooted<ExternRef>> {
    trace!("read_stdin");
    let mut input = String::new();
    let n = io::stdin().read_line(&mut input)?;
    ExternRef::new(
        caller,
        if n == 0 {
            input
        } else {
            // XXX: should add this newline?
            input.push('\n');
            input
        },
    )
}
fn io_file_random_access(_caller: Caller<'_, HootStatus>, _a: Rooted<ExternRef>) -> Result<i32> {
    trace!("file_random_access");
    todo!()
}
fn rt_flonum_to_string(_caller: Caller<'_, HootStatus>, _param: f64) -> Result<Rooted<ExternRef>> {
    trace!("flonum_to_string");
    todo!()
}
fn rt_bignum_from_u64(caller: Caller<'_, HootStatus>, param: i64) -> Result<Rooted<ExternRef>> {
    trace!("bignum_from_u64 {:?}", param);
    let num: u64 = param as u64;
    ExternRef::new(caller, BigInt::from(num))
}
fn rt_bignum_from_i64(caller: Caller<'_, HootStatus>, param: i64) -> Result<Rooted<ExternRef>> {
    trace!("bignum_from_i64 {:?} {:?}", param, BigInt::from(param));
    ExternRef::new(caller, BigInt::from(param))
}
fn rt_bignum_from_u32(caller: Caller<'_, HootStatus>, param: u32) -> Result<Rooted<ExternRef>> {
    trace!("bignum_from_u32");
    ExternRef::new(caller, BigInt::from(param))
}

fn rt_bignum_from_i32(caller: Caller<'_, HootStatus>, param: i32) -> Result<Rooted<ExternRef>> {
    trace!("bignum_from_i32");
    ExternRef::new(caller, BigInt::from(param))
}

fn rt_bignum_sub(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    _b: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_sub");
    let a_int = extern_ref_to_bigint(a, &caller)?;
    let b_int = extern_ref_to_bigint(a, &caller)?;
    ExternRef::new(caller, a_int - b_int)
}

fn rt_bignum_sub_i32(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    b: i32,
) -> Result<Rooted<ExternRef>> {
    let a_int = extern_ref_to_bigint(a, &caller)?;
    let result = a_int - b;
    trace!("bignum_sub_i32 {:?}", &result);
    ExternRef::new(caller, result)
}

fn rt_die(_caller: Caller<'_, HootStatus>, _param: Rooted<ExternRef>, _eq: Rooted<EqRef>) {
    trace!("die");
}

fn io_file_buffer_ref(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: i32,
) -> Result<i32> {
    trace!("file_buffer_ref");
    todo!()
}
fn io_file_buffer_set(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: i32,
    _c: i32,
) -> Result<()> {
    trace!("file_buffer_set");
    todo!()
}

fn io_file_buffer_size(_caller: Caller<'_, HootStatus>, _a: Rooted<ExternRef>) -> Result<i32> {
    trace!("file_buffer_size");
    todo!()
}

fn rt_bignum_is_i64(caller: Caller<'_, HootStatus>, param: Rooted<ExternRef>) -> Result<i32> {
    trace!("bignum_is_i64");
    let nu = extern_ref_to_bigint(param, &caller)?;
    Ok(bool_to_i32(nu.to_i64().is_some()))
}
fn rt_bignum_get_i64(caller: Caller<'_, HootStatus>, param: Rooted<ExternRef>) -> Result<i64> {
    trace!("bignum_get_i64");
    let nu = extern_ref_to_bigint(param, &caller)?;

    let out = nu.to_i64().unwrap_or_else(|| {
        if nu < BigInt::ZERO {
            i64::MIN
        } else {
            i64::MAX
        }
    });
    return Ok(out);
}

fn rt_bignum_mul(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    b: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_mul");
    let a_int = extern_ref_to_bigint(a, &caller)?;
    let b_int = extern_ref_to_bigint(b, &caller)?;
    ExternRef::new(caller, a_int * b_int)
}

fn rt_bignum_mul_i32(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    b: i32,
) -> Result<Rooted<ExternRef>> {
    let a_int = extern_ref_to_bigint(a, &caller)?;
    let ret = &a_int * b;
    trace!("bignum_mul_i32 {:?} {:?} {:?}", &a_int, &b, &ret);
    ExternRef::new(caller, ret)
}
fn rt_bignum_gcd(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    b: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_gcd");
    let a_int = extern_ref_to_bigint(a, &caller)?;
    let b_int = extern_ref_to_bigint(b, &caller)?;
    let ret = a_int.gcd(&b_int);
    ExternRef::new(caller, ret)
}

fn rt_bignum_quo(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    b: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_quo");
    let a_int = extern_ref_to_bigint(a, &caller)?;
    let b_int = extern_ref_to_bigint(b, &caller)?;
    ExternRef::new(caller, BigInt::from(a_int / b_int))
}

fn rt_quit(_caller: Caller<'_, HootStatus>, a: i32) -> Result<()> {
    trace!("quit {:?}", a);
    exit(a);
}
fn rt_stream_make_chunk(
    caller: Caller<'_, HootStatus>,
    _a: i32,
    _b: i64,
) -> Result<Rooted<ExternRef>> {
    trace!("stream_make_chunk");
    ExternRef::new(caller, "")
}

fn rt_bignum_logxor_i32(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _param: i32,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_logxor_i32");
    todo!()
}

fn io_write_stdout(caller: Caller<'_, HootStatus>, string: Rooted<ExternRef>) -> Result<()> {
    let nu = extern_ref_to_string(string, &caller)?;
    trace!("write_stdout {:?}", &nu);
    let mut stdout = io::stdout().lock();
    stdout.write_all(nu.as_bytes())?;
    stdout.flush()?;
    Ok(())
}
fn io_write_stderr(caller: Caller<'_, HootStatus>, string: Rooted<ExternRef>) -> Result<()> {
    let nu = extern_ref_to_string(string, &caller)?;
    trace!("write_stderr {:?}", &nu);
    let mut stderr = io::stderr().lock();
    stderr.write_all(nu.as_bytes())?;
    stderr.flush()?;
    Ok(())
}

fn io_open_input_file(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
) -> Result<Option<Rooted<ExternRef>>> {
    trace!("open_input_file");
    todo!()
}

fn io_open_output_file(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("open_output_file");
    todo!()
}
fn io_file_exists(_caller: Caller<'_, HootStatus>, _a: Rooted<ExternRef>) -> Result<i32> {
    trace!("io_file_exists");
    todo!()
}

fn io_delete_file(_caller: Caller<'_, HootStatus>, _a: Rooted<ExternRef>) -> Result<()> {
    trace!("io_delete_file");
    todo!()
}

fn io_write_file(_caller: Caller<'_, HootStatus>, _a: Rooted<ExternRef>, _b: i32) -> Result<i32> {
    trace!("io_write_file");
    todo!()
}

fn rt_string_upcase(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    let c: String = extern_ref_to_string(a, &caller)?;
    let ret = c.to_uppercase();

    trace!("string_upcase {:?}", &ret);
    ExternRef::new(caller, ret)
}

fn rt_string_downcase(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    let c: String = extern_ref_to_string(a, &caller)?;
    let ret = c.to_lowercase();
    trace!("string_downcase {:?}", &ret);
    ExternRef::new(caller, ret)
}

fn rt_stream_read(
    _caller: Caller<'_, HootStatus>,
    _param: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("stream_read");
    todo!();
}

fn debug_code_name(caller: Caller<'_, HootStatus>, _param: Func) -> Option<Rooted<ExternRef>> {
    trace!("code_name");
    let aaa: String = "TODO_debug::code_name".into();
    let cc = ExternRef::new(caller, aaa).unwrap();
    Some(cc.clone())
}

fn rt_make_regexp(
    _caller: Caller<'_, HootStatus>,
    _param: Rooted<ExternRef>,
    _b: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("make_regexp");
    todo!()
}
fn rt_regexp_match_start(_caller: Caller<'_, HootStatus>, _a: Rooted<ExternRef>) -> i32 {
    trace!("regexp_match_start");
    todo!()
}
fn rt_weak_ref_deref(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: Rooted<EqRef>,
) -> Result<Rooted<EqRef>> {
    trace!("weak_ref_deref");
    todo!()
}
fn rt_bignum_logxor(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_logxor");
    todo!()
}
fn rt_bignum_mod(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_mod");
    todo!()
}
fn rt_bignum_to_f64(_caller: Caller<'_, HootStatus>, _a: Rooted<ExternRef>) -> f64 {
    trace!("bignum_to_f64");
    todo!()
}
fn rt_bignum_eq_f64(caller: Caller<'_, HootStatus>, a: Rooted<ExternRef>, _b: f64) -> Result<i32> {
    trace!("bignum_eq_f64");
    let _a_int = extern_ref_to_bigint(a, &caller)?;
    todo!()
}
fn rt_bignum_eq(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    b: Rooted<ExternRef>,
) -> Result<i32> {
    trace!("bignum_eq");
    let a_int = extern_ref_to_bigint(a, &caller)?;
    let b_int = extern_ref_to_bigint(b, &caller)?;
    Ok(bool_to_i32(a_int == b_int))
}

fn rt_bignum_rsh(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    b: i64,
) -> Result<Rooted<ExternRef>> {
    let nu = extern_ref_to_bigint(a, &caller)?;
    let ret = &nu >> b;
    trace!("bignum_rsh {:?} {:?} {:?}", &nu, &b, &ret);
    ExternRef::new(caller, ret)
}

fn rt_bignum_logand(
    caller: Caller<'_, HootStatus>,
    _param: Rooted<ExternRef>,
    _b: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_logand");
    ExternRef::new(caller, "")
}
fn rt_bignum_logand_i32(
    caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _param: i32,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_logand_i32");
    ExternRef::new(caller, "")
}
fn rt_bignum_lt_big_f64(_caller: Caller<'_, HootStatus>, _b: Rooted<ExternRef>, _c: f64) -> i32 {
    trace!("bignum_lt_big_f64");
    todo!()
}

fn finalization_finalization_registry_register(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: Rooted<EqRef>,
    _c: Rooted<EqRef>,
) {
    trace!("finalization_registry_register");
    todo!()
}

fn rt_bignum_lt_f64_big(
    _caller: Caller<'_, HootStatus>,
    _c: f64,
    _param: Rooted<ExternRef>,
) -> Result<i32> {
    trace!("bignum_lt_f64_big");
    todo!()
}
fn rt_bignum_le_f64_big(
    _caller: Caller<'_, HootStatus>,
    _b: f64,
    _param: Rooted<ExternRef>,
) -> i32 {
    trace!("bignum_le_f64_big");
    todo!()
}
fn rt_bignum_lsh_i32_i64(
    caller: Caller<'_, HootStatus>,
    param: i32,
    b: i64,
) -> Result<Rooted<ExternRef>> {
    let n: BigInt = BigInt::from(param);
    let c = n << b;
    trace!("bignum_lsh_i32_i64 {:?}", &c);
    ExternRef::new(caller, c)
}
fn rt_bignum_lt_i32_big(
    caller: Caller<'_, HootStatus>,
    a: i32,
    b: Rooted<ExternRef>,
) -> Result<i32> {
    let b_int = extern_ref_to_bigint(b, &caller)?;
    trace!("bignum_lt_i32_big");
    Ok(bool_to_i32(BigInt::from(a) < b_int))
}

fn rt_bignum_le_big_f64(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    b: f64,
) -> Result<i32> {
    let a_int = extern_ref_to_bigint(a, &caller)?;
    trace!("bignum_le_big_f64");
    let a_f64 = a_int
        .to_f64()
        .ok_or_else(|| anyhow::anyhow!("bigint cannot fit into f64"))?;

    Ok(bool_to_i32(a_f64 <= b))
}
fn rt_bignum_lt_big_i32(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    b: i32,
) -> Result<i32> {
    trace!("bignum_lt_big_i32");
    let a_int: BigInt = extern_ref_to_bigint(a, &caller)?;
    Ok(bool_to_i32(a_int < b.into()))
}
fn rt_bignum_is_u64(caller: Caller<'_, HootStatus>, a: Rooted<ExternRef>) -> Result<i32> {
    let a_int = extern_ref_to_bigint(a, &caller)?;
    trace!("bignum_is_u64");
    Ok(bool_to_i32(a_int.to_u64().is_some()))
}

fn ffi_is_extern_func(caller: Caller<'_, HootStatus>, _c: Rooted<ExternRef>) -> Result<i32> {
    trace!("ffi_is_extern_func: {:?}", caller.data());
    todo!()
}

fn ffi_procedure_to_extern(
    caller: Caller<'_, HootStatus>,
    _c: Rooted<EqRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("ffi_procedure_to_extern: {:?}", caller.data());
    todo!()
}

fn ffi_call_extern(
    caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: Rooted<EqRef>,
) -> Result<Rooted<EqRef>> {
    trace!("ffi_call_extern: {:?}", caller.data());
    todo!()
}

fn rt_bignum_rem(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    b: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    let a_int: BigInt = extern_ref_to_bigint(a, &caller)?;
    let b_int: BigInt = extern_ref_to_bigint(b, &caller)?;
    let ret = &a_int % &b_int;
    trace!("bignum_rem {:?} {:?} ret: {:?}", &a_int, &b_int, &ret);
    ExternRef::new(caller, ret)
}

fn rt_bignum_logior_i32(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _param: i32,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_logior_i32");
    todo!()
}
fn rt_bignum_logior(
    _caller: Caller<'_, HootStatus>,
    _param: Rooted<ExternRef>,
    _b: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_logior");
    todo!()
}

fn rt_regexp_match_count(_caller: Caller<'_, HootStatus>, _param: Rooted<ExternRef>) -> i32 {
    trace!("regexp_match_count");

    todo!()
}
fn finalization_finalization_registry_unregister(
    _caller: Caller<'_, HootStatus>,
    _param: Rooted<ExternRef>,
    _b: Rooted<EqRef>,
) -> i32 {
    trace!("finalization_registry_unregister");
    todo!();
}
fn rt_regexp_match_end(_caller: Caller<'_, HootStatus>, _a: Rooted<ExternRef>) -> i32 {
    trace!("regexp_match_end");
    todo!();
}
fn rt_regexp_match_substring(
    _caller: Caller<'_, HootStatus>,
    _b: Rooted<ExternRef>,
    _param: i32,
) -> Option<Rooted<ExternRef>> {
    trace!("regexp_match_substring");
    todo!();
}
fn rt_make_weak_map(caller: Caller<'_, HootStatus>) -> Result<Rooted<ExternRef>> {
    let c = WeakMap::new(20);
    trace!("make_weak_map");
    ExternRef::new(caller, c)
}
fn debug_debug_str_i32(_caller: Caller<'_, HootStatus>, _param: i32) -> Result<()> {
    trace!("debug_str_i32");
    todo!();
}
fn debug_debug_str(_caller: Caller<'_, HootStatus>, _a: Rooted<ExternRef>) -> Result<()> {
    trace!("debug_str");
    todo!();
}

fn debug_debug_str_scm(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: Rooted<EqRef>,
) -> Result<()> {
    trace!("debug_str_scm");
    todo!();
}

fn debug_code_source(
    _caller: Caller<'_, HootStatus>,
    _param: Func,
) -> (Option<Rooted<ExternRef>>, i32, i32) {
    trace!("code_source");
    todo!()
}
fn finalization_finalization_registry_register_with_token(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _param: Rooted<EqRef>,
    _c: Rooted<EqRef>,
    _d: Rooted<EqRef>,
) -> Result<()> {
    trace!("finalization_registry_register_with_token");
    todo!()
}
fn finalization_make_finalization_registry(
    _caller: Caller<'_, HootStatus>,
    _param: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("make_finalization_registry");
    todo!();
}
fn rt_bignum_lt(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    b: Rooted<ExternRef>,
) -> Result<i32> {
    let a_int = extern_ref_to_bigint(a, &caller)?;
    let b_int = extern_ref_to_bigint(b, &caller)?;
    let ret = a_int < b_int;
    trace!("bignum_lt, {:?} {:?} {:?}", &a_int, &b_int, a_int < b_int);
    Ok(bool_to_i32(ret))
}
fn rt_bignum_add_i32(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    b: i32,
) -> Result<Rooted<ExternRef>> {
    let a_int = extern_ref_to_bigint(a, &caller)?;
    let result = a_int + b;
    trace!("bignum_add_i32 {:?}", &result);
    ExternRef::new(caller, result)
}
fn rt_bignum_add(
    caller: Caller<'_, HootStatus>,
    a: Rooted<ExternRef>,
    _b: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_add");
    let a_int = extern_ref_to_bigint(a, &caller)?;
    let b_int = extern_ref_to_bigint(a, &caller)?;
    ExternRef::new(caller, a_int + b_int)
}
fn rt_bignum_lsh(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: i64,
) -> Result<Rooted<ExternRef>> {
    trace!("bignum_lsh");
    todo!()
}

fn rt_bignum_from_string(
    caller: Caller<'_, HootStatus>,
    str: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    let cc = str.data(&caller)?.expect("wtf");
    let st = cc
        .downcast_ref::<String>()
        .ok_or_else(|| anyhow::anyhow!("externref is not String!"))?;
    let nu: BigInt = st.parse().unwrap();
    trace!("bignum_from_string {:?}", &st);
    ExternRef::new(caller, nu)
}
fn rt_make_weak_ref(
    _caller: Caller<'_, HootStatus>,
    _param: Rooted<EqRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("make_weak_ref");
    todo!();
}

fn rt_weak_map_set(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: Rooted<EqRef>,
    _c: Rooted<EqRef>,
) -> Result<()> {
    trace!("rt_weak_map_set");
    todo!();
}

fn rt_weak_map_get(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: Rooted<EqRef>,
    _c: Rooted<EqRef>,
) -> Result<Rooted<EqRef>> {
    trace!("rt_weak_map_get");
    todo!();
}

fn rt_current_second(_caller: Caller<'_, HootStatus>) -> Result<f64> {
    trace!("rt_current_second");
    todo!();
}
fn rt_current_jiffy(_caller: Caller<'_, HootStatus>) -> Result<f64> {
    trace!("rt_current_jiffy");
    todo!();
}
fn rt_jiffies_per_second(_caller: Caller<'_, HootStatus>) -> Result<i32> {
    trace!("rt_jiffies_per_second");
    todo!();
}
fn rt_regexp_exec(
    _caller: Caller<'_, HootStatus>,
    _a: Rooted<ExternRef>,
    _b: Rooted<ExternRef>,
) -> Option<Rooted<ExternRef>> {
    trace!("regexp_exec");
    todo!()
}
fn rt_regexp_match_string(
    _caller: Caller<'_, HootStatus>,
    _param: Rooted<ExternRef>,
) -> Result<Rooted<ExternRef>> {
    trace!("regexp_match_string");
    todo!()
}

fn rt_ftodo(_caller: Caller<'_, HootStatus>, _a: f64) -> Result<f64> {
    trace!("a TODO fn");
    todo!()
}

fn rt_fatan2(_caller: Caller<'_, HootStatus>, _a: f64, _b: f64) -> Result<f64> {
    trace!("rt_fatan2");
    todo!()
}

fn rt_bignum_logcount(_caller: Caller<'_, HootStatus>, _a: Rooted<ExternRef>) -> Result<i32> {
    trace!("rt_bignum_logcount");
    todo!()
}
fn init_trace() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .try_init();
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_trace();
    let rt = Runtime::new(None)?;
    let engine = rt.engine;
    let mut store = Store::new(&engine, HootStatus::new(30));
    let mut linker = Linker::new(&engine);
    let module = Module::from_file(&engine, cli.filename)?;
    {
        let rt_string_to_wtf8_ty = module
            .imports()
            .find(|i| i.module() == "rt" && i.name() == "string_to_wtf8")
            .ok_or_else(|| anyhow!("import not found"))?;

        let rt_string_to_wtf8_func_ty: FuncType = match rt_string_to_wtf8_ty.ty() {
            ExternType::Func(f) => f.clone(),
            other => bail!("expected func import, got {other:?}"),
        };
        let rt_string_to_wtf8_result_val_ty = rt_string_to_wtf8_func_ty
            .results()
            .nth(0)
            .ok_or_else(|| anyhow!("missing result"))?
            .clone();

        let rt_string_to_wtf8_result_ref_ty = match rt_string_to_wtf8_result_val_ty {
            ValType::Ref(r) => r,
            _ => bail!("result is not a ref type"),
        };

        let concrete_array_ty = match rt_string_to_wtf8_result_ref_ty.heap_type() {
            HeapType::ConcreteArray(arr_ty) => arr_ty.clone(),
            ht => bail!("result heap type is not a concrete array: {ht:?}"),
        };
        let allocator = ArrayRefPre::new(&mut store, concrete_array_ty);
        linker.func_new(
            "rt",
            "string_to_wtf8",
            rt_string_to_wtf8_func_ty,
            move |mut caller, _params, results| {
                trace!("string_to_wtf8");
                let m: String = match _params[0] {
                    Val::ExternRef(Some(rooted)) => extern_ref_to_string(rooted, &caller)?,
                    _ => todo!(),
                };
                let vals: Vec<Val> = m.as_bytes().iter().map(|&b| Val::I32(b as i32)).collect();
                let arr = ArrayRef::new_fixed(&mut caller, &allocator, &vals)?;
                results[0] = Val::AnyRef(Some(arr.to_anyref()));
                Ok(())
            },
        )?;
    }

    {
        let rt_wtf8_to_string_ty = module
            .imports()
            .find(|i| i.module() == "rt" && i.name() == "wtf8_to_string")
            .ok_or_else(|| anyhow!("import not found"))?;

        let rt_wtf8_to_string_func_ty: FuncType = match rt_wtf8_to_string_ty.ty() {
            ExternType::Func(f) => f,
            other => bail!("expected func import, got {other:?}"),
        };

        linker.func_new(
            "rt",
            "wtf8_to_string",
            rt_wtf8_to_string_func_ty,
            move |mut caller, _params, results| {
                let any = _params[0];
                let ret = match any {
                    Val::AnyRef(Some(rooted)) => {
                        let array = rooted
                            .clone()
                            .as_eqref(&caller)?
                            .ok_or_else(|| anyhow::anyhow!("not eqref?"))?
                            .as_array(&caller)?
                            .ok_or_else(|| anyhow::anyhow!("not array?"))?;
                        let len = array.len(&caller)?;
                        let mut bytes: Vec<u8> = Vec::with_capacity(len as usize);
                        for i in 0..len {
                            let elem = array.get(&mut caller, i)?;
                            match elem {
                                Val::I32(x) => {
                                    bytes.push((x & 0xff) as u8);
                                }
                                _ => {
                                    bail!("wtf, not i32")
                                }
                            }
                        }
                        let s = String::from_utf8(bytes)?;
                        s
                    }
                    _ => todo!(),
                };

                trace!("run wtf8_to_string {:?}", &ret);
                let ext_ref = ExternRef::new(&mut caller, ret.clone())?;
                results[0] = Val::ExternRef(Some(ext_ref));
                Ok(())
            },
        )?;
    }

    linker.func_wrap("debug", "code_name", debug_code_name)?;
    linker.func_wrap("debug", "code_source", debug_code_source)?;
    linker.func_wrap("debug", "debug_str", debug_debug_str)?;
    linker.func_wrap("debug", "debug_str_i32", debug_debug_str_i32)?;
    linker.func_wrap("debug", "debug_str_scm", debug_debug_str_scm)?;

    linker.func_wrap("ffi", "call_extern", ffi_call_extern)?;
    linker.func_wrap("ffi", "is_extern_func", ffi_is_extern_func)?;
    linker.func_wrap("ffi", "procedure_to_extern", ffi_procedure_to_extern)?;

    linker.func_wrap("io", "close_file", io_close_file)?;
    linker.func_wrap("io", "file_buffer_ref", io_file_buffer_ref)?;
    linker.func_wrap("io", "file_buffer_set", io_file_buffer_set)?;
    linker.func_wrap("io", "file_buffer_size", io_file_buffer_size)?;
    linker.func_wrap("io", "file_random_access", io_file_random_access)?;
    linker.func_wrap("io", "open_input_file", io_open_input_file)?;
    linker.func_wrap("io", "open_output_file", io_open_output_file)?;
    linker.func_wrap("io", "file_exists", io_file_exists)?;
    linker.func_wrap("io", "delete_file", io_delete_file)?;
    linker.func_wrap("io", "write_file", io_write_file)?;
    linker.func_wrap("io", "read_file", io_read_file)?;
    linker.func_wrap("io", "read_stdin", io_read_stdin)?;
    linker.func_wrap("io", "seek_file", io_seek_file)?;
    linker.func_wrap("io", "write_stderr", io_write_stderr)?;
    linker.func_wrap("io", "write_stdout", io_write_stdout)?;

    linker.func_wrap("rt", "bignum_add", rt_bignum_add)?;
    linker.func_wrap("rt", "bignum_add_i32", rt_bignum_add_i32)?;
    linker.func_wrap("rt", "bignum_eq", rt_bignum_eq)?;
    linker.func_wrap("rt", "bignum_eq_f64", rt_bignum_eq_f64)?;
    linker.func_wrap("rt", "bignum_from_i32", rt_bignum_from_i32)?;
    linker.func_wrap("rt", "bignum_from_i64", rt_bignum_from_i64)?;
    linker.func_wrap("rt", "bignum_from_string", rt_bignum_from_string)?;
    linker.func_wrap("rt", "bignum_from_u32", rt_bignum_from_u32)?;
    linker.func_wrap("rt", "bignum_from_u64", rt_bignum_from_u64)?;
    linker.func_wrap("rt", "bignum_gcd", rt_bignum_gcd)?;
    linker.func_wrap("rt", "bignum_get_i64", rt_bignum_get_i64)?;
    linker.func_wrap("rt", "bignum_is_i64", rt_bignum_is_i64)?;
    linker.func_wrap("rt", "bignum_is_u64", rt_bignum_is_u64)?;
    linker.func_wrap("rt", "bignum_le_big_f64", rt_bignum_le_big_f64)?;
    linker.func_wrap("rt", "bignum_le_f64_big", rt_bignum_le_f64_big)?;
    linker.func_wrap("rt", "bignum_logand", rt_bignum_logand)?;
    linker.func_wrap("rt", "bignum_logand_i32", rt_bignum_logand_i32)?;
    linker.func_wrap("rt", "bignum_logcount", rt_bignum_logcount)?;
    linker.func_wrap("rt", "bignum_logior", rt_bignum_logior)?;
    linker.func_wrap("rt", "bignum_logior_i32", rt_bignum_logior_i32)?;
    linker.func_wrap("rt", "bignum_logxor", rt_bignum_logxor)?;
    linker.func_wrap("rt", "bignum_logxor_i32", rt_bignum_logxor_i32)?;
    linker.func_wrap("rt", "bignum_lsh", rt_bignum_lsh)?;
    linker.func_wrap("rt", "bignum_lsh_i32_i64", rt_bignum_lsh_i32_i64)?;
    linker.func_wrap("rt", "bignum_lt", rt_bignum_lt)?;
    linker.func_wrap("rt", "bignum_lt_big_f64", rt_bignum_lt_big_f64)?;
    linker.func_wrap("rt", "bignum_lt_big_i32", rt_bignum_lt_big_i32)?;
    linker.func_wrap("rt", "bignum_lt_f64_big", rt_bignum_lt_f64_big)?;
    linker.func_wrap("rt", "bignum_lt_i32_big", rt_bignum_lt_i32_big)?;
    linker.func_wrap("rt", "bignum_mod", rt_bignum_mod)?;
    linker.func_wrap("rt", "bignum_mul", rt_bignum_mul)?;
    linker.func_wrap("rt", "bignum_mul_i32", rt_bignum_mul_i32)?;
    linker.func_wrap("rt", "bignum_quo", rt_bignum_quo)?;
    linker.func_wrap("rt", "bignum_rem", rt_bignum_rem)?;
    linker.func_wrap("rt", "bignum_rsh", rt_bignum_rsh)?;
    linker.func_wrap("rt", "bignum_sub", rt_bignum_sub)?;
    linker.func_wrap("rt", "bignum_sub_i32", rt_bignum_sub_i32)?;
    linker.func_wrap("rt", "bignum_to_f64", rt_bignum_to_f64)?;
    linker.func_wrap("rt", "current_jiffy", rt_current_jiffy)?;
    linker.func_wrap("rt", "current_second", rt_current_second)?;
    linker.func_wrap("rt", "die", rt_die)?;
    linker.func_wrap("rt", "facos", rt_ftodo)?;
    linker.func_wrap("rt", "fasin", rt_ftodo)?;
    linker.func_wrap("rt", "fatan", rt_ftodo)?;
    linker.func_wrap("rt", "fatan2", rt_fatan2)?;
    linker.func_wrap("rt", "fcos", rt_ftodo)?;
    linker.func_wrap("rt", "fexp", rt_ftodo)?;
    linker.func_wrap("rt", "flog", rt_ftodo)?;
    linker.func_wrap("rt", "flonum_to_string", rt_flonum_to_string)?;
    linker.func_wrap("rt", "fsin", rt_ftodo)?;
    linker.func_wrap("rt", "ftan", rt_ftodo)?;
    linker.func_wrap("rt", "jiffies_per_second", rt_jiffies_per_second)?;
    linker.func_wrap("rt", "make_regexp", rt_make_regexp)?;
    linker.func_wrap("rt", "make_weak_map", rt_make_weak_map)?;
    linker.func_wrap("rt", "make_weak_ref", rt_make_weak_ref)?;
    linker.func_wrap("rt", "quit", rt_quit)?;
    linker.func_wrap("rt", "regexp_exec", rt_regexp_exec)?;
    linker.func_wrap("rt", "regexp_match_count", rt_regexp_match_count)?;
    linker.func_wrap("rt", "regexp_match_end", rt_regexp_match_end)?;
    linker.func_wrap("rt", "regexp_match_start", rt_regexp_match_start)?;
    linker.func_wrap("rt", "regexp_match_string", rt_regexp_match_string)?;
    linker.func_wrap("rt", "regexp_match_substring", rt_regexp_match_substring)?;
    linker.func_wrap("rt", "stream_make_chunk", rt_stream_make_chunk)?;
    linker.func_wrap("rt", "stream_read", rt_stream_read)?;
    linker.func_wrap("rt", "string_downcase", rt_string_downcase)?;
    linker.func_wrap("rt", "string_upcase", rt_string_upcase)?;
    linker.func_wrap("rt", "weak_map_get", rt_weak_map_get)?;
    linker.func_wrap("rt", "weak_map_set", rt_weak_map_set)?;
    linker.func_wrap("rt", "weak_ref_deref", rt_weak_ref_deref)?;
    linker.func_wrap(
        "finalization",
        "finalization_registry_register",
        finalization_finalization_registry_register,
    )?;
    linker.func_wrap(
        "finalization",
        "make_finalization_registry",
        finalization_make_finalization_registry,
    )?;
    linker.func_wrap(
        "finalization",
        "finalization_registry_unregister",
        finalization_finalization_registry_unregister,
    )?;
    linker.func_wrap(
        "finalization",
        "finalization_registry_register_with_token",
        finalization_finalization_registry_register_with_token,
    )?;
    let c = linker.instantiate(&mut store, &module)?;
    let f = c.get_typed_func::<(), ()>(&mut store, "main")?;
    f.call(&mut store, ())?;
    Ok(())
}
