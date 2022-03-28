use ms_toollib::{
    cal_is_op_possibility_cells, cal_possibility, cal_possibility_onboard, is_guess_while_needless,
    is_solvable, mark_board, solve_direct, solve_enumerate, is_able_to_solve,
};
use ms_toollib::{cal_table_minenum_recursion, combine, refresh_matrix, refresh_matrixs};

// 测试各种引擎类的函数

#[test]
fn cal_is_op_possibility_cells_works() {
    // 测试开空概率计算函数
    let game_board = vec![
        vec![10, 10, 1, 1, 10, 1, 0, 0],
        vec![10, 10, 1, 10, 10, 3, 2, 1],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 2, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
    ];
    let ans = cal_is_op_possibility_cells(&game_board, 20.0, &vec![[0, 0], [1, 1], [1, 6], [7, 2]]);
    print!("{:?}", ans)
}

#[test]
fn solve_direct_works() {
    // 测试枚举判雷引擎
    let mut game_board = vec![
        vec![
            10, 10, 10, 1, 1, 0, 0, 1, 11, 1, 0, 0, 0, 0, 1, 10, 10, 10, 2, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 11, 2, 0, 0, 1, 1, 1, 1, 2, 2, 1, 1, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 11, 4, 2, 1, 0, 0, 0, 1, 11, 11, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 11, 11, 2, 1, 0, 0, 2, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 10, 10, 11, 3, 1, 1, 1, 11, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10, 10,
        ],
    ];
    let (mut matrix_as, mut matrix_xs, mut matrix_bs, _, _) = refresh_matrixs(&game_board);
    let ans = solve_direct(
        &mut matrix_as,
        &mut matrix_xs,
        &mut matrix_bs,
        &mut game_board,
    );
    print!("{:?}", ans)
}

#[test]
fn solve_enumerate_works() {
    // 测试枚举判雷引擎
    let mut game_board = vec![
        vec![0, 0, 1, 10, 10, 10, 10, 10],
        vec![0, 0, 2, 10, 10, 10, 10, 10],
        vec![1, 1, 3, 11, 10, 10, 10, 10],
        vec![10, 10, 4, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
    ];
    let (matrix_as, matrix_xs, matrix_bs, _, _) = refresh_matrixs(&game_board);
    let ans = solve_enumerate(&matrix_as, &matrix_xs, &matrix_bs);
    print!("{:?}", ans)
}

#[test]
fn cal_possibility_onboard_1_works() {
    // 测试概率计算引擎
    let mut game_board = vec![
        vec![10, 10, 1, 1, 10, 1, 0, 0],
        vec![10, 10, 1, 10, 10, 3, 2, 1],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 2, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
    ];
    let ans = cal_possibility(&game_board, 10.0);
    print!("设置雷数为10，概率计算引擎的结果为：{:?}", ans);
    let ans = cal_possibility(&game_board, 0.15625);
    print!("设置雷的比例为15.625%，概率计算引擎的结果为：{:?}", ans);
    // 对局面预标记，以加速计算
    mark_board(&mut game_board);
    let ans = cal_possibility_onboard(&game_board, 10.0);
    print!("设置雷的比例为10，与局面位置对应的概率结果为：{:?}", ans);
}

#[test]
fn cal_possibility_onboard_2_works() {
    // 测试概率计算引擎
    let game_board = vec![
        vec![
            0, 1, 10, 3, 10, 2, 1, 10, 2, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10, 10,
        ],
        vec![
            0, 1, 1, 4, 10, 3, 2, 2, 3, 3, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10,
        ],
        vec![
            0, 0, 0, 2, 10, 3, 2, 11, 1, 3, 10, 10, 3, 1, 1, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10,
        ],
        vec![
            0, 0, 0, 1, 2, 10, 2, 1, 1, 2, 11, 10, 2, 0, 1, 10, 2, 1, 2, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10,
        ],
        vec![
            1, 1, 0, 0, 1, 2, 2, 1, 0, 1, 2, 10, 1, 0, 1, 1, 1, 0, 1, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10,
        ],
        vec![
            10, 2, 0, 0, 0, 2, 10, 3, 2, 3, 3, 10, 2, 1, 0, 0, 0, 0, 2, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10,
        ],
        vec![
            10, 3, 0, 0, 1, 3, 10, 3, 10, 10, 10, 11, 11, 1, 1, 2, 2, 1, 1, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10,
        ],
        vec![
            10, 2, 0, 1, 3, 10, 3, 2, 3, 10, 4, 4, 3, 2, 2, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10,
        ],
        vec![
            1, 2, 2, 4, 10, 10, 3, 0, 1, 1, 1, 1, 11, 2, 3, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10,
        ],
        vec![
            0, 2, 10, 10, 10, 10, 3, 0, 0, 0, 0, 1, 1, 2, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10,
        ],
        vec![
            1, 3, 11, 6, 10, 10, 2, 0, 0, 0, 0, 0, 0, 1, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10,
        ],
        vec![
            1, 11, 2, 3, 10, 3, 1, 0, 1, 1, 1, 1, 2, 2, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10,
        ],
        vec![
            1, 2, 2, 2, 1, 2, 1, 1, 1, 11, 1, 1, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10,
        ],
        vec![
            2, 3, 11, 1, 0, 2, 10, 4, 3, 2, 1, 1, 3, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10,
        ],
        vec![
            10, 10, 2, 1, 1, 4, 10, 10, 11, 1, 0, 0, 1, 1, 1, 1, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10, 10,
        ],
        vec![
            2, 2, 1, 0, 1, 10, 10, 4, 2, 1, 0, 0, 0, 0, 0, 1, 10, 10, 10, 10, 10, 10, 10, 10, 10,
            10, 10, 10, 10, 10,
        ],
    ];
    let ans = cal_possibility(&game_board, 10.0);
    let ans = cal_possibility(&game_board, 0.15625);
    print!("{:?}", ans)
}

#[test]
fn cal_table_minenum_recursion_works() {
    // 测试递归枚举引擎
    //     [[0, 0, 1, -1, 2, 1, 1, -1], [0, 0, 2, 3, -1, 3, 3, 2], [1, 1, 3, -1, 4, -1, -1, 2], [2, -1, 4, -1, 3, 4, -1, 4], [3, -1, 5, 2, 1, 3, -1, -1], [3, -1, -1, 2, 1, 2, -1, 3], [-1, 5, 4, -1,
    // 1, 1, 2, 2], [-1, 3, -1, 2, 1, 0, 1, -1]]
    let game_board = vec![
        vec![0, 0, 1, 10, 10, 10, 10, 10],
        vec![0, 0, 2, 10, 10, 10, 10, 10],
        vec![1, 1, 3, 11, 10, 10, 10, 10],
        vec![10, 10, 4, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
        vec![10, 10, 10, 10, 10, 10, 10, 10],
    ];
    // let a = isSolvable(&board, 0, 0, 40);
    // print!("{:?}", a);
    let (matrix_a, matrix_x, matrix_b) = refresh_matrix(&game_board);
    let (matrix_a_s, matrix_x_s, combination_relationship) = combine(&matrix_a, &matrix_x);
    let table = cal_table_minenum_recursion(
        &matrix_a_s,
        &matrix_x_s,
        &matrix_b,
        &combination_relationship,
    );
    println!("table的结果为：{:?}", table);
}

#[test]
fn is_guess_while_needless_works() {
    // let mut game_board = vec![
    //     vec![0, 0, 1, 10, 10, 10, 10, 10],
    //     vec![0, 0, 2, 10, 10, 10, 10, 10],
    //     vec![1, 1, 3, 11, 10, 10, 10, 10],
    //     vec![10, 10, 4, 10, 10, 10, 10, 10],
    //     vec![10, 10, 10, 10, 10, 10, 10, 10],
    //     vec![10, 10, 10, 10, 10, 10, 10, 10],
    //     vec![10, 10, 10, 10, 10, 10, 10, 10],
    //     vec![10, 10, 10, 10, 10, 10, 10, 10],
    // ];
    let mut game_board = vec![
        vec![ 1, 10, 10,  2, 10,  2, 1, 0],
        vec![10, 10, 10, 10, 10, 10, 1, 0],
        vec![10, 10, 10, 10, 10,  3, 1, 0],
        vec![10, 10, 10, 10,  1,  1, 0, 0],
        vec![10, 10, 10, 10,  1,  0, 0, 0],
        vec![10, 10, 10, 10,  1,  0, 0, 0],
        vec![10, 10, 10, 10,  2,  1, 0, 0],
        vec![10, 10, 10, 10, 10,  1, 0, 0],
    ];
    let code = is_guess_while_needless(&mut game_board, &(3, 2));
    println!("{:?}", code);
    let code = is_guess_while_needless(&mut game_board, &(0, 1));
    println!("{:?}", code);
    let code = is_guess_while_needless(&mut game_board, &(0, 4));
    println!("{:?}", code);
    // let code = is_guess_while_needless(&mut game_board, &(0, 3));
    // println!("{:?}", code);
}

#[test]
fn is_able_to_solve_works() {
let mut game_board = vec![
        vec![11, 10, 10, 10, 1, 0, 1, 10], 
        vec![11, 10, 2, 2, 1, 0, 2, 10], 
        vec![11, 10, 1, 0, 0, 0, 2, 10], 
        vec![11, 11, 1, 0, 0, 0, 1, 10], 
        vec![1, 1, 1, 1, 2, 2, 1, 10], 
        vec![0, 0, 0, 10, 10, 10, 10, 10], 
        vec![0, 0, 0, 1, 2, 2, 2, 10], 
        vec![0, 0, 0, 0, 0, 0, 1, 10]
    ];
    let code = is_able_to_solve(&mut game_board, &(4, 3));
    println!("{:?}", code);
}










