use std::{
    fs::read_to_string,
    path::Path,
};

use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};

macro_rules! benchmark_day {
    ($year:ident, $day:ident, $c:ident) => {{
        let input = {
            let year = stringify!($year);
            let day = stringify!($day);
            let path = Path::new("input")
                .join(year)
                .join(day)
                .with_extension("txt");
            let raw = read_to_string(&path).expect(&format!(
                "Missing input file! Please place input in {}",
                path.display()
            ));
            raw.trim().to_string()
        };

        $c.bench_function(
            &format!("{}/{}/part1", stringify!($year), stringify!($day)),
            |b| {
                b.iter(|| {
                    let parsed_input = advent_of_code::$year::$day::parse(&input);
                    advent_of_code::$year::$day::part1(&parsed_input)
                })
            },
        );

        $c.bench_function(
            &format!("{}/{}/part2", stringify!($year), stringify!($day)),
            |b| {
                b.iter(|| {
                    let parsed_input = advent_of_code::$year::$day::parse(&input);
                    advent_of_code::$year::$day::part2(&parsed_input)
                })
            },
        );
    }};
}

macro_rules! benchmark_year {
    ($year:ident => $($day:ident),*) => {
        fn $year(c: &mut Criterion) {
            $(
                benchmark_day!($year, $day, c);
            )*
        }
    };
}

benchmark_year!(template_year => template_day);

benchmark_year!(year2015 =>
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark_year!(year2016 =>
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark_year!(year2017 =>
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark_year!(year2018 =>
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark_year!(year2019 =>
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark_year!(year2020 =>
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark_year!(year2021 =>
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark_year!(year2022 =>
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark_year!(year2023 =>
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark_year!(year2024 =>
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark_year!(year2025 =>
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12
);

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = template_year,
        year2015, year2016, year2017, year2018, year2019, year2020,
        year2021, year2022, year2023, year2024, year2025
);

criterion_main!(benches);
