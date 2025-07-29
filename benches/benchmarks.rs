use std::{
    fs::read_to_string,
    path::Path,
    sync::LazyLock,
};

use criterion::{
    Criterion,
    criterion_group,
    criterion_main,
};

macro_rules! benchmark {
    ($year:tt $($day:tt),*) => {
        $(
            paste::paste! {
                fn [<$year _ $day>](c: &mut Criterion) {
                    use advent_of_code::$year::$day as solution;

                    static INPUT: LazyLock<String> = LazyLock::new(|| {
                        let year = stringify!($year);
                        let day = stringify!($day);
                        let path = Path::new("input").join(year).join(day).with_extension("txt");
                        let raw = read_to_string(&path).expect(&format!("Missing input file! Please place input in {}", &path.display()));
                        raw.trim().to_string()
                    });

                    let benchmark_name = format!("{}/{}/part1", stringify!($year), stringify!($day));
                    c.bench_function(&benchmark_name, |b| {
                        b.iter(|| {
                            let input = solution::parse(&INPUT);
                            solution::part1(&input)
                        })
                    });

                    let benchmark_name = format!("{}/{}/part2", stringify!($year), stringify!($day));
                    c.bench_function(&benchmark_name, |b| {
                        b.iter(|| {
                            let input = solution::parse(&INPUT);
                            solution::part2(&input)
                        })
                    });
                }
            }
        )*
    }
}

benchmark!(template_year
    template_day
);

benchmark!(year2015
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark!(year2016
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark!(year2017
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark!(year2018
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark!(year2019
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark!(year2020
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark!(year2021
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark!(year2022
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark!(year2023
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

benchmark!(year2024
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day14, day15, day16, day17, day18, day19, day20,
    day21, day22, day23, day24, day25
);

criterion_group!(
    benches,
    template_year_template_day,
    year2015_day01,
    year2015_day02,
    year2015_day03,
    year2015_day04,
    year2015_day05,
    year2015_day06,
    year2015_day07,
    year2015_day08,
    year2015_day09,
    year2015_day10,
    year2015_day11,
    year2015_day12,
    year2015_day13,
    year2015_day14,
    year2015_day15,
    year2015_day16,
    year2015_day17,
    year2015_day18,
    year2015_day19,
    year2015_day20,
    year2015_day21,
    year2015_day22,
    year2015_day23,
    year2015_day24,
    year2015_day25,
    year2016_day01,
    year2016_day02,
    year2016_day03,
    year2016_day04,
    year2016_day05,
    year2016_day06,
    year2016_day07,
    year2016_day08,
    year2016_day09,
    year2016_day10,
    year2016_day11,
    year2016_day12,
    year2016_day13,
    year2016_day14,
    year2016_day15,
    year2016_day16,
    year2016_day17,
    year2016_day18,
    year2016_day19,
    year2016_day20,
    year2016_day21,
    year2016_day22,
    year2016_day23,
    year2016_day24,
    year2016_day25,
    year2017_day01,
    year2017_day02,
    year2017_day03,
    year2017_day04,
    year2017_day05,
    year2017_day06,
    year2017_day07,
    year2017_day08,
    year2017_day09,
    year2017_day10,
    year2017_day11,
    year2017_day12,
    year2017_day13,
    year2017_day14,
    year2017_day15,
    year2017_day16,
    year2017_day17,
    year2017_day18,
    year2017_day19,
    year2017_day20,
    year2017_day21,
    year2017_day22,
    year2017_day23,
    year2017_day24,
    year2017_day25,
    year2018_day01,
    year2018_day02,
    year2018_day03,
    year2018_day04,
    year2018_day05,
    year2018_day06,
    year2018_day07,
    year2018_day08,
    year2018_day09,
    year2018_day10,
    year2018_day11,
    year2018_day12,
    year2018_day13,
    year2018_day14,
    year2018_day15,
    year2018_day16,
    year2018_day17,
    year2018_day18,
    year2018_day19,
    year2018_day20,
    year2018_day21,
    year2018_day22,
    year2018_day23,
    year2018_day24,
    year2018_day25,
    year2019_day01,
    year2019_day02,
    year2019_day03,
    year2019_day04,
    year2019_day05,
    year2019_day06,
    year2019_day07,
    year2019_day08,
    year2019_day09,
    year2019_day10,
    year2019_day11,
    year2019_day12,
    year2019_day13,
    year2019_day14,
    year2019_day15,
    year2019_day16,
    year2019_day17,
    year2019_day18,
    year2019_day19,
    year2019_day20,
    year2019_day21,
    year2019_day22,
    year2019_day23,
    year2019_day24,
    year2019_day25,
    year2020_day01,
    year2020_day02,
    year2020_day03,
    year2020_day04,
    year2020_day05,
    year2020_day06,
    year2020_day07,
    year2020_day08,
    year2020_day09,
    year2020_day10,
    year2020_day11,
    year2020_day12,
    year2020_day13,
    year2020_day14,
    year2020_day15,
    year2020_day16,
    year2020_day17,
    year2020_day18,
    year2020_day19,
    year2020_day20,
    year2020_day21,
    year2020_day22,
    year2020_day23,
    year2020_day24,
    year2020_day25,
    year2021_day01,
    year2021_day02,
    year2021_day03,
    year2021_day04,
    year2021_day05,
    year2021_day06,
    year2021_day07,
    year2021_day08,
    year2021_day09,
    year2021_day10,
    year2021_day11,
    year2021_day12,
    year2021_day13,
    year2021_day14,
    year2021_day15,
    year2021_day16,
    year2021_day17,
    year2021_day18,
    year2021_day19,
    year2021_day20,
    year2021_day21,
    year2021_day22,
    year2021_day23,
    year2021_day24,
    year2021_day25,
    year2022_day01,
    year2022_day02,
    year2022_day03,
    year2022_day04,
    year2022_day05,
    year2022_day06,
    year2022_day07,
    year2022_day08,
    year2022_day09,
    year2022_day10,
    year2022_day11,
    year2022_day12,
    year2022_day13,
    year2022_day14,
    year2022_day15,
    year2022_day16,
    year2022_day17,
    year2022_day18,
    year2022_day19,
    year2022_day20,
    year2022_day21,
    year2022_day22,
    year2022_day23,
    year2022_day24,
    year2022_day25,
    year2023_day01,
    year2023_day02,
    year2023_day03,
    year2023_day04,
    year2023_day05,
    year2023_day06,
    year2023_day07,
    year2023_day08,
    year2023_day09,
    year2023_day10,
    year2023_day11,
    year2023_day12,
    year2023_day13,
    year2023_day14,
    year2023_day15,
    year2023_day16,
    year2023_day17,
    year2023_day18,
    year2023_day19,
    year2023_day20,
    year2023_day21,
    year2023_day22,
    year2023_day23,
    year2023_day24,
    year2023_day25,
    year2024_day01,
    year2024_day02,
    year2024_day03,
    year2024_day04,
    year2024_day05,
    year2024_day06,
    year2024_day07,
    year2024_day08,
    year2024_day09,
    year2024_day10,
    year2024_day11,
    year2024_day12,
    year2024_day13,
    year2024_day14,
    year2024_day15,
    year2024_day16,
    year2024_day17,
    year2024_day18,
    year2024_day19,
    year2024_day20,
    year2024_day21,
    year2024_day22,
    year2024_day23,
    year2024_day24,
    year2024_day25
);

criterion_main!(benches);
