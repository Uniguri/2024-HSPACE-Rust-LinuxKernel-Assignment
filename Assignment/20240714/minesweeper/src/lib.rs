pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let height = minefield.len();
    let width = if height >= 1 { minefield[0].len() } else { 0 };
    let mut ret_bytes = vec![vec![b' '; width]; height];

    const DIRECTION_SIZE: usize = 8;
    const DX: [i32; DIRECTION_SIZE] = [-1, 0, 1, 1, 1, 0, -1, -1];
    const DY: [i32; DIRECTION_SIZE] = [1, 1, 1, 0, -1, -1, -1, 0];

    for y in 0..height {
        let line = minefield[y].as_bytes();
        for x in 0..width {
            let cur = line[x];
            match cur {
                b' ' => (),
                b'*' => {
                    ret_bytes[y][x] = b'*';
                    for i in 0..DIRECTION_SIZE {
                        if let (Some(new_x), Some(new_y)) = (
                            x.checked_add_signed(DX[i] as isize),
                            y.checked_add_signed(DY[i] as isize),
                        ) {
                            if new_x < width && new_y < height {
                                let cur = &mut ret_bytes[new_y][new_x];
                                if *cur == b' ' {
                                    *cur = b'1';
                                } else if *cur != b'*' {
                                    *cur += 1;
                                }
                            }
                        }
                    }
                }
                _ => (),
            }
        }
    }

    let string_vectors: Result<Vec<String>, _> =
        ret_bytes.into_iter().map(String::from_utf8).collect();

    return string_vectors.unwrap();
}

#[cfg(test)]
fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

#[cfg(test)]
fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

#[cfg(test)]
fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}

#[test]
fn no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}

#[test]
fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}

#[test]
fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}

#[test]
fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}

#[test]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}

#[test]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}

#[test]
fn horizontal_line() {
    #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
}

#[test]
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}

#[test]
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}

#[test]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}

#[test]
fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}

#[test]
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}
