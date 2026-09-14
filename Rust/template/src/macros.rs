#[allow(unused)]
#[cfg(feature = "bench")]
macro_rules! benchmark {
    ($benchmark_name:expr, $code_block:block) => {{
        let start = std::time::Instant::now();
        let result = $code_block;
        println!("Benchmark: [{}] {:?}", $benchmark_name, start.elapsed());
        result
    }};
}

#[allow(unused)]
#[cfg(not(feature = "bench"))]
macro_rules! benchmark {
    ($benchmark_name:expr, $code_block:block) => {{
        $code_block
    }};
}

#[allow(unused)]
#[cfg(feature = "bench")]
macro_rules! group_benchmark {
    ($group_name:ident, $code_block:block) => {{
        let mut $group_name: std::collections::HashMap<&str, std::time::Duration> =
            std::collections::HashMap::new();
        let result = $code_block;
        for (bench_name, bench_time) in $group_name.iter() {
            println!("Benchmark: [{}] Total time: {:?}", bench_name, bench_time);
        }
        result
    }};
}

#[allow(unused)]
#[cfg(not(feature = "bench"))]
macro_rules! group_benchmark {
    ($group_name:ident, $code_block:block) => {{
        $code_block
    }};
}

#[allow(unused)]
#[cfg(feature = "bench")]
macro_rules! benchmark_multiple {
    ($group_name:ident, $benchmark_name:expr, $code_block:block) => {{
        let start = std::time::Instant::now();
        let result = $code_block;
        *$group_name.entry($benchmark_name).or_default() += start.elapsed();
        result
    }};
}

#[allow(unused)]
#[cfg(not(feature = "bench"))]
macro_rules! benchmark_multiple {
    ($group_name:ident, $benchmark_name:expr, $code_block:block) => {{
        $code_block
    }};
}
