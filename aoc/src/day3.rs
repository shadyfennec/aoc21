use crate::AocDay;

pub struct Day3;

impl Default for Day3 {
    fn default() -> Self {
        Self {}
    }
}

fn rate_bits(values: &[Vec<u8>], bit: usize) -> (usize, usize) {
    values
        .iter()
        .map(|v| {
            v.iter()
                .map(|b| if *b == 0 { (1, 0) } else { (0, 1) })
                .collect::<Vec<_>>()
        })
        .fold((0, 0), |mut bits, values| {
            bits.0 += values[bit].0;
            bits.1 += values[bit].1;
            bits
        })
}

enum FilterPredicate {
    Min,
    Max,
}

fn filter_values(
    values: &[Vec<u8>],
    rating: (usize, usize),
    bit: usize,
    predicate: FilterPredicate,
) -> Vec<Vec<u8>> {
    let target = match predicate {
        FilterPredicate::Min => {
            if rating.1 < rating.0 {
                1
            } else {
                0
            }
        }
        FilterPredicate::Max => {
            if rating.0 <= rating.1 {
                1
            } else {
                0
            }
        }
    };

    values
        .iter()
        .filter(|v| v[bit] == target)
        .cloned()
        .collect()
}

impl AocDay for Day3 {
    fn title(&self) -> String {
        "Binary Diagnostic".to_owned()
    }

    fn inputs(&self) -> &[&'static str] {
        &["inputs/day3/small.txt", "inputs/day3/real.txt"]
    }

    fn part_1(
        &self,
        input: Vec<String>,
        output: std::sync::mpsc::Sender<String>,
        _debug: std::sync::mpsc::Sender<String>,
    ) -> color_eyre::eyre::Result<()> {
        let len = input[0].len();

        let result = input
            .into_iter()
            .map(|i| {
                i.chars()
                    .map(|c| match c {
                        '0' => (1, 0),
                        '1' => (0, 1),
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .fold(Vec::new(), |mut vec, bits| {
                if vec.is_empty() {
                    vec = bits
                } else {
                    vec.iter_mut().zip(bits.iter()).for_each(|(a, b)| {
                        a.0 += b.0;
                        a.1 += b.1;
                    });
                }
                vec
            });

        let mask = u64::MAX >> (64 - len);

        let gamma = u64::from_str_radix(
            &result
                .iter()
                .map(|c| if c.0 > c.1 { '0' } else { '1' })
                .collect::<String>(),
            2,
        )
        .unwrap();

        let epsilon = (!gamma) & mask;

        self.println(format!("{}", gamma * epsilon), &output);
        Ok(())
    }

    fn part_2(
        &self,
        input: Vec<String>,
        output: std::sync::mpsc::Sender<String>,
        debug: std::sync::mpsc::Sender<String>,
    ) -> color_eyre::eyre::Result<()> {
        let input = input
            .into_iter()
            .map(|s| {
                s.chars()
                    .map(|i| match i {
                        '0' => 0,
                        '1' => 1,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut v_max = input;
        let mut v_min = v_max.clone();

        for bit in 0.. {
            match (v_max.len(), v_min.len()) {
                (x, y) if x == 0 || y == 0 => unreachable!(),
                (1, 1) => break,
                (1, _) => {
                    v_min =
                        filter_values(&v_min, rate_bits(&v_min, bit), bit, FilterPredicate::Min);
                }
                (_, 1) => {
                    v_max =
                        filter_values(&v_max, rate_bits(&v_max, bit), bit, FilterPredicate::Max);
                }
                (_, _) => {
                    v_min =
                        filter_values(&v_min, rate_bits(&v_min, bit), bit, FilterPredicate::Min);
                    v_max =
                        filter_values(&v_max, rate_bits(&v_max, bit), bit, FilterPredicate::Max);
                }
            }
            // self.println(format!("v_max: {:?}\nv_min:{:?}\n", v_max, v_min), &debug);
        }

        let oxy = u32::from_str_radix(
            &v_max[0]
                .iter()
                .map(|b| if *b == 0 { '0' } else { '1' })
                .collect::<String>(),
            2,
        )
        .unwrap();

        let co2 = u32::from_str_radix(
            &v_min[0]
                .iter()
                .map(|b| if *b == 0 { '0' } else { '1' })
                .collect::<String>(),
            2,
        )
        .unwrap();

        self.println(format!("{}, {}, {}", oxy, co2, oxy * co2), &output);

        Ok(())
    }
}
