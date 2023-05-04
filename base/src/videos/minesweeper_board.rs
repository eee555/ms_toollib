use crate::utils::refresh_board;

use crate::safe_board::{BoardSize, SafeBoard};
use std::cmp::{max, min};

/// 没有时间、像素观念的局面状态机，侧重分析操作与局面的交互、推衍局面。在线地统计左右双击次数、ce次数、左键、右键、双击、当前解决的3BV。  
/// - 局限：不关注具体的线路（没有像素观念），因此不能计算path等。  
/// - 注意：ce的计算与扫雷网是不同的，本工具箱中，重复标同一个雷只算一个ce，即反复标雷、取消标雷不算作ce。
/// 应用场景：强化学习训练AI、游戏复盘计算指标。不能处理高亮（18）、算法确定是雷（12）等标记。  
/// - 用python调用时的示例：
/// ```python
/// import ms_toollib as ms
/// board = [
///     [0, 0, 1, -1, 2, 1, 1, -1],
///     [0, 0, 2, 3, -1, 3, 3, 2],
///     [1, 1, 3, -1, 4, -1, -1, 2],
///     [2, -1, 4, -1, 3, 4, -1, 4],
///     [3, -1, 5, 2, 1, 3, -1, -1],
///     [3, -1, -1, 2, 1, 2, -1, 3],
///     [-1, 5, 4, -1, 1, 1, 2, 2],
///     [-1, 3, -1, 2, 1, 0, 1, -1],
///     ];
/// v = ms.MinesweeperBoard(board) # 实例化后再用
/// v.step('lc', (0, 0)) # 左键按下
/// v.step('lr', (0, 0)) # 左键弹起
/// print('左键次数: ', v.left)
/// print('右键次数: ', v.right)
/// print('ce数: ', v.ces)
/// print('标雷数: ', v.flag)
/// print('解决3BV数: ', v.solved3BV)
/// print('局面: ', v.game_board)
/// ```
pub struct MinesweeperBoard<T> {
    pub board: T,
    /// 局面
    pub game_board: Vec<Vec<i32>>,
    flagedList: Vec<(usize, usize)>, // 记录哪些雷曾经被标过，则再标这些雷不记为ce
    /// 左键数
    pub left: usize,
    /// 右键数
    pub right: usize,
    /// 双击数
    pub double: usize,
    /// ce数
    pub ce: usize,
    /// 标雷数
    pub flag: usize,
    /// 已解决的3BV数
    pub bbbv_solved: usize,
    pub row: usize,
    pub column: usize,
    pub mouse_state: MouseState,
    pub game_board_state: GameBoardState,
    // 指针，用于判断局面是否全部扫开
    pointer_x: usize,
    pointer_y: usize,
    pre_flag_num: usize,
    // 中键是否按下，配合“m”、“mc”、“mr”。
    middle_hold: bool,
}

impl Default for MinesweeperBoard<Vec<Vec<i32>>> {
    fn default() -> Self {
        MinesweeperBoard {
            board: vec![],
            game_board: vec![],
            flagedList: vec![],
            left: 0,
            right: 0,
            double: 0,
            ce: 0,
            flag: 0,
            bbbv_solved: 0,
            row: 0,
            column: 0,
            mouse_state: MouseState::Undefined,
            game_board_state: GameBoardState::Ready,
            pointer_x: 0,
            pointer_y: 0,
            pre_flag_num: 0,
            middle_hold: false,
        }
    }
}

impl Default for MinesweeperBoard<SafeBoard> {
    fn default() -> Self {
        MinesweeperBoard {
            board: SafeBoard::new(vec![]),
            game_board: vec![],
            flagedList: vec![],
            left: 0,
            right: 0,
            double: 0,
            ce: 0,
            flag: 0,
            bbbv_solved: 0,
            row: 0,
            column: 0,
            mouse_state: MouseState::Undefined,
            game_board_state: GameBoardState::Ready,
            pointer_x: 0,
            pointer_y: 0,
            pre_flag_num: 0,
            middle_hold: false,
        }
    }
}

impl MinesweeperBoard<Vec<Vec<i32>>> {
    pub fn new(board: Vec<Vec<i32>>) -> MinesweeperBoard<Vec<Vec<i32>>> {
        let row = board.get_row();
        let column = board.get_column();
        MinesweeperBoard {
            board,
            row,
            column,
            game_board: vec![vec![10; column]; row],
            flagedList: vec![],
            mouse_state: MouseState::UpUp,
            ..MinesweeperBoard::<Vec<Vec<i32>>>::default()
        }
    }
    /// 初始化。对应强化学习领域gym的api中的reset。
    pub fn reset(&mut self) {
        self.game_board = vec![vec![10; self.column]; self.row];
        self.board = vec![vec![0; self.column]; self.row];
        self.left = 0;
        self.right = 0;
        self.double = 0;
        self.ce = 0;
        self.flag = 0;
        self.left = 0;
        self.bbbv_solved = 0;
        self.flagedList = vec![];
        self.mouse_state = MouseState::UpUp;
        self.game_board_state = GameBoardState::Ready;
        self.pointer_x = 0;
        self.pointer_y = 0;
    }
}

impl MinesweeperBoard<SafeBoard> {
    pub fn new(board: SafeBoard) -> MinesweeperBoard<SafeBoard> {
        let row = board.get_row();
        let column = board.get_column();
        MinesweeperBoard {
            board,
            row,
            column,
            game_board: vec![vec![10; column]; row],
            flagedList: vec![],
            mouse_state: MouseState::UpUp,
            ..MinesweeperBoard::<SafeBoard>::default()
        }
    }
}

impl<T> MinesweeperBoard<T> {
    /// Playing状态下的左击，没有按下抬起之分
    fn left_click(&mut self, x: usize, y: usize) -> Result<u8, ()>
    where
        T: std::ops::Index<usize> + BoardSize,
        T::Output: std::ops::Index<usize, Output = i32>,
    {
        self.left += 1;
        if self.game_board[x][y] != 10 {
            return Ok(0);
        }
        refresh_board(&self.board, &mut self.game_board, vec![(x, y)]);
        match self.board[x][y] {
            0 => {
                self.bbbv_solved += 1;
                self.ce += 1;
                // refresh_board(&self.board, &mut self.game_board, vec![(x, y)]);
                if self.is_win() {
                    self.game_board_state = GameBoardState::Win;
                }
                Ok(2)
            }
            -1 => {
                // refresh_board(&self.board, &mut self.game_board, vec![(x, y)]);
                self.game_board_state = GameBoardState::Loss;
                Ok(0)
            }
            _ => {
                // refresh_board(&self.board, &mut self.game_board, vec![(x, y)]);
                if self.num_is_3BV(x, y) {
                    self.bbbv_solved += 1;
                }
                self.ce += 1;
                if self.is_win() {
                    self.game_board_state = GameBoardState::Win;
                }
                Ok(2)
            }
        }
    }
    /// Playing状态下的右击，没有按下抬起之分
    fn right_click(&mut self, x: usize, y: usize) -> Result<u8, ()>
    where
        T: std::ops::Index<usize> + BoardSize + std::fmt::Debug,
        T::Output: std::ops::Index<usize, Output = i32>,
    {
        self.right += 1;
        if self.game_board[x][y] < 10 {
            return Ok(0);
        } else {
            // println!("{:?}", self.board);
            if self.board[x][y] != -1 {
                match self.game_board[x][y] {
                    10 => {
                        self.game_board[x][y] = 11;
                        self.flag += 1;
                    }
                    11 => {
                        self.game_board[x][y] = 10;
                        self.flag -= 1;
                    }
                    _ => return Err(()),
                }
            } else {
                match self.game_board[x][y] {
                    10 => {
                        self.game_board[x][y] = 11;
                        self.flag += 1;
                        if !self.flagedList.contains(&(x, y)) {
                            self.ce += 1;
                        }
                        self.flagedList.push((x, y));
                    }
                    11 => {
                        self.game_board[x][y] = 10;
                        self.flag -= 1;
                    }
                    _ => return Err(()),
                }
            }
            Ok(1)
        }
    }
    /// Playing状态下的双击，没有按下抬起之分
    fn chording_click(&mut self, x: usize, y: usize) -> Result<u8, ()>
    where
        T: std::ops::Index<usize> + BoardSize,
        T::Output: std::ops::Index<usize, Output = i32>,
    {
        self.double += 1;
        if self.game_board[x][y] == 0 || self.game_board[x][y] >= 8 {
            return Ok(0);
        }
        let mut flagChordingUseful = false; // 双击有效的基础上，周围是否有未打开的格子
        let mut chordingCells = vec![]; // 未打开的格子的集合
        let mut flagedNum = 0; // 双击点周围的标雷数
        let mut surround3BV = 0; // 周围的3BV
        let mut flag_ch_op = false; // 是否通过双击开空了：一次双击最多打开一个空
        for i in max(1, x) - 1..min(self.row, x + 2) {
            for j in max(1, y) - 1..min(self.column, y + 2) {
                if i != x || j != y {
                    if self.game_board[i][j] == 11 {
                        flagedNum += 1
                    }
                    if self.game_board[i][j] == 10 {
                        chordingCells.push((i, j));
                        flagChordingUseful = true;
                        if self.board[i][j] > 0 {
                            if self.num_is_3BV(i, j) {
                                surround3BV += 1;
                            }
                        } else if self.board[i][j] == 0 {
                            flag_ch_op = true;
                        }
                    }
                }
            }
        }
        if flagedNum == self.game_board[x][y] && flagChordingUseful {
            self.ce += 1;
            self.bbbv_solved += surround3BV;
            if flag_ch_op {
                self.bbbv_solved += 1;
            }
            for ch in &chordingCells {
                if self.board[ch.0][ch.1] == -1 {
                    self.game_board_state = GameBoardState::Loss;
                }
            }
            refresh_board(&self.board, &mut self.game_board, chordingCells);
            if self.is_win() {
                self.game_board_state = GameBoardState::Win;
            }
            Ok(3)
        } else {
            Ok(0)
        }
    }
    fn num_is_3BV(&self, x: usize, y: usize) -> bool
    where
        T: std::ops::Index<usize> + BoardSize,
        T::Output: std::ops::Index<usize, Output = i32>,
    {
        // 判断该大于0的数字是不是3BV
        // 如果是0，即使是3bv，依然返回false
        if self.board[x][y] <= 0 {
            return false;
        }
        for i in max(1, x) - 1..min(self.row, x + 2) {
            for j in max(1, y) - 1..min(self.column, y + 2) {
                if self.board[i][j] == 0 {
                    return false;
                }
            }
        }
        true
    }
    /// 返回的值的含义是：0：没有任何作用的操作，例如左键数字、踩雷。  
    /// 1：推进了局面，但没有改变ai对局面的判断，特指标雷。  
    /// 2：改变局面的操作，左键、双击。
    /// 3: 正确的双击.   
    /// e的类型有11种，lc（左键按下）, lr（左键抬起）, rc（右键按下）, rr（右键抬起）, mc（中键按下）, mr（中键抬起）,   
    ///     cc（双键按下，但不确定是哪个键），pf（在开始前预先标雷，而又失去了标记的过程）,   
    ///     l（左键按下或抬起）, r（右键按下或抬起）, m（中键按下或抬起）。  
    /// ## 注意事项：
    /// - 在理想的鼠标状态机中，有些情况是不可能的，例如右键没有抬起就按下两次，但在阿比特中就观察到这种事情。
    // 局面外按下的事件，以及连带的释放一律对鼠标状态没有任何影响，UI框架不会激活回调
    pub fn step(&mut self, e: &str, pos: (usize, usize)) -> Result<u8, ()>
    where
        T: std::ops::Index<usize> + BoardSize + std::fmt::Debug,
        T::Output: std::ops::Index<usize, Output = i32>,
    {
        // println!("e: {:?}", e);
        if pos.0 == self.row && pos.1 == self.column && (e == "rc" || e == "lc" || e == "cc") {
            // 这里按理应该报错，局面外的按下不该进来
            return Ok(0);
        }
        match self.game_board_state {
            GameBoardState::Ready => match e {
                "mv" => {
                    return Ok(0);
                }
                "lc" => {
                    //  "l"处理不了，很复杂，以后再说
                    match self.mouse_state {
                        MouseState::UpUp => {
                            self.game_board_state = GameBoardState::PreFlaging;
                            self.mouse_state = MouseState::DownUp
                        }
                        MouseState::UpDown => self.mouse_state = MouseState::Chording,
                        MouseState::UpDownNotFlag => self.mouse_state = MouseState::ChordingNotFlag,
                        _ => return Err(()),
                    }
                    return Ok(0);
                }
                "pf" => {
                    assert!(
                        self.game_board[pos.0][pos.1] == 10,
                        "按定义，pf不能在标雷上执行。请报告这个奇怪的录像。"
                    );
                    self.pre_flag_num += 1;
                    self.game_board_state = GameBoardState::PreFlaging;
                    return self.right_click(pos.0, pos.1);
                }
                //  "r"处理不了，很复杂，以后再说
                "rc" => match self.mouse_state {
                    MouseState::UpUp => {
                        self.pre_flag_num = 1;
                        self.game_board_state = GameBoardState::PreFlaging;
                        self.mouse_state = MouseState::UpDown;
                        return self.right_click(pos.0, pos.1);
                    }
                    MouseState::DownUpAfterChording => {
                        self.mouse_state = MouseState::Chording;
                        return Ok(0);
                    }
                    _ => return Err(()),
                },
                "lr" => match self.mouse_state {
                    MouseState::Chording | MouseState::ChordingNotFlag => {
                        self.mouse_state = MouseState::UpDown;
                        return Ok(0);
                    }
                    MouseState::DownUpAfterChording => {
                        self.mouse_state = MouseState::UpUp;
                        return Ok(0);
                    }
                    _ => return Err(()),
                },
                "rr" => match self.mouse_state {
                    MouseState::UpDown => {
                        self.mouse_state = MouseState::UpUp;
                        return Ok(0);
                    }
                    MouseState::Chording => {
                        self.mouse_state = MouseState::DownUpAfterChording;
                        return Ok(0);
                    }
                    _ => return Err(()),
                },
                "cc" => {
                    match self.mouse_state {
                        MouseState::DownUp => self.mouse_state = MouseState::Chording,
                        MouseState::DownUpAfterChording => self.mouse_state = MouseState::Chording,
                        MouseState::UpDown => self.mouse_state = MouseState::Chording,
                        MouseState::UpDownNotFlag => self.mouse_state = MouseState::ChordingNotFlag,
                        _ => return Err(()),
                    }
                    return Ok(0);
                }
                _ => return Err(()),
            },
            GameBoardState::PreFlaging => match e {
                "lc" | "rr" | "mv" => {} // 和playing状态下恰好一致，主要是计算点击次数
                "lr" => match self.mouse_state {
                    MouseState::DownUp => {
                        if pos.0 == self.row && pos.1 == self.column {
                            self.mouse_state = MouseState::UpUp;
                            if self.pre_flag_num == 0 {
                                self.game_board_state = GameBoardState::Ready;
                                self.clear_click_num();
                            }
                            return Ok(0);
                        }
                        if self.game_board[pos.0][pos.1] == 10 {
                            self.game_board_state = GameBoardState::Playing;
                        } else {
                            return Ok(0);
                        }
                    }
                    MouseState::Chording
                    | MouseState::DownUpAfterChording
                    | MouseState::ChordingNotFlag
                    | MouseState::UpUp
                    | MouseState::Undefined => {}
                    _ => return Err(()),
                },
                "pf" => {
                    assert!(
                        self.game_board[pos.0][pos.1] == 10,
                        "按定义，pf不能在标雷上执行。请报告这个奇怪的录像。"
                    );
                    self.pre_flag_num += 1;
                    return self.right_click(pos.0, pos.1);
                }
                "rc" => match self.mouse_state {
                    MouseState::UpUp => {
                        self.mouse_state = MouseState::UpDown;
                        if self.game_board[pos.0][pos.1] == 10 {
                            self.pre_flag_num += 1;
                            self.game_board_state = GameBoardState::PreFlaging;
                            return self.right_click(pos.0, pos.1);
                        } else {
                            self.pre_flag_num -= 1;
                            if self.pre_flag_num == 0 {
                                self.game_board_state = GameBoardState::Ready;
                                self.flag = 0;
                                self.flagedList.clear();
                                self.double = 0;
                                self.left = 0;
                                self.right = 0;
                                self.game_board[pos.0][pos.1] = 10;
                                return Ok(0);
                            } else {
                                return self.right_click(pos.0, pos.1);
                            }
                        }
                    }
                    MouseState::DownUp => {
                        if self.pre_flag_num == 0 {
                            self.game_board_state = GameBoardState::Ready;
                        }
                        self.mouse_state = MouseState::Chording
                    }
                    MouseState::DownUpAfterChording => self.mouse_state = MouseState::Chording,
                    _ => return Err(()),
                },
                "cc" => {
                    match self.mouse_state {
                        MouseState::DownUp => {
                            if self.pre_flag_num == 0 {
                                self.game_board_state = GameBoardState::Ready;
                            }
                            self.mouse_state = MouseState::Chording;
                        }
                        MouseState::DownUpAfterChording => self.mouse_state = MouseState::Chording,
                        MouseState::UpDown => self.mouse_state = MouseState::Chording,
                        MouseState::UpDownNotFlag => self.mouse_state = MouseState::ChordingNotFlag,
                        _ => return Err(()),
                    }
                    return Ok(0);
                }
                _ => return Err(()),
            },
            GameBoardState::Playing => {}
            _ => return Ok(0),
        }
        match e {
            "lc" => match self.mouse_state {
                MouseState::UpUp => self.mouse_state = MouseState::DownUp,
                MouseState::UpDown => self.mouse_state = MouseState::Chording,
                MouseState::UpDownNotFlag => self.mouse_state = MouseState::ChordingNotFlag,
                // 以下情况其实是不可能的
                MouseState::DownUp => {}
                MouseState::DownUpAfterChording => {}
                MouseState::Chording => {}
                MouseState::ChordingNotFlag => {}
                MouseState::Undefined => self.mouse_state = MouseState::DownUp,
            },
            "lr" => match self.mouse_state {
                MouseState::DownUp => {
                    self.mouse_state = MouseState::UpUp;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    // println!("x={:?}, y={:?}", pos.0, pos.1);
                    return self.left_click(pos.0, pos.1);
                }
                MouseState::Chording => {
                    self.mouse_state = MouseState::UpDown;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    return self.chording_click(pos.0, pos.1);
                }
                MouseState::DownUpAfterChording => self.mouse_state = MouseState::UpUp,
                MouseState::ChordingNotFlag => {
                    self.mouse_state = MouseState::UpDown;
                    self.right -= 1;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    return self.chording_click(pos.0, pos.1);
                }
                // 以下情况其实是不可能的
                MouseState::UpDown => {}
                MouseState::UpDownNotFlag => {}
                MouseState::UpUp => self.mouse_state = MouseState::UpUp,
                MouseState::Undefined => self.mouse_state = MouseState::UpUp,
            },
            "l" => match self.mouse_state {
                MouseState::DownUp => {
                    self.mouse_state = MouseState::UpUp;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    // println!("x={:?}, y={:?}", pos.0, pos.1);
                    return self.left_click(pos.0, pos.1);
                }
                MouseState::Chording => {
                    self.mouse_state = MouseState::UpDown;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    return self.chording_click(pos.0, pos.1);
                }
                MouseState::DownUpAfterChording => self.mouse_state = MouseState::UpUp,
                MouseState::ChordingNotFlag => {
                    self.mouse_state = MouseState::UpDown;
                    self.right -= 1;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    return self.chording_click(pos.0, pos.1);
                }
                MouseState::UpUp => self.mouse_state = MouseState::DownUp,
                MouseState::UpDown => self.mouse_state = MouseState::Chording,
                MouseState::UpDownNotFlag => self.mouse_state = MouseState::ChordingNotFlag,
                MouseState::Undefined => self.mouse_state = MouseState::UpUp,
            },
            "rc" => match self.mouse_state {
                MouseState::UpUp => {
                    if self.game_board[pos.0][pos.1] < 10 {
                        self.mouse_state = MouseState::UpDownNotFlag;
                    } else {
                        self.mouse_state = MouseState::UpDown;
                    }
                    return self.right_click(pos.0, pos.1);
                }
                MouseState::DownUp => self.mouse_state = MouseState::Chording,
                MouseState::DownUpAfterChording => self.mouse_state = MouseState::Chording,
                // 以下情况其实是不可能的
                MouseState::UpDown => {}
                MouseState::UpDownNotFlag => {}
                MouseState::Chording => {}
                MouseState::ChordingNotFlag => {}
                MouseState::Undefined => self.mouse_state = MouseState::UpDown,
            },
            "rr" => match self.mouse_state {
                MouseState::UpDown => self.mouse_state = MouseState::UpUp,
                MouseState::UpDownNotFlag => self.mouse_state = MouseState::UpUp,
                MouseState::Chording => {
                    self.mouse_state = MouseState::DownUpAfterChording;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    return self.chording_click(pos.0, pos.1);
                }
                MouseState::ChordingNotFlag => {
                    self.mouse_state = MouseState::DownUpAfterChording;
                    self.right -= 1;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    return self.chording_click(pos.0, pos.1);
                }
                // 以下情况其实是不可能的
                MouseState::DownUp => {}
                MouseState::DownUpAfterChording => {}
                MouseState::UpUp => self.mouse_state = MouseState::UpUp,
                MouseState::Undefined => self.mouse_state = MouseState::UpUp,
            },
            "r" => match self.mouse_state {
                MouseState::UpDown => self.mouse_state = MouseState::UpUp,
                MouseState::UpDownNotFlag => self.mouse_state = MouseState::UpUp,
                MouseState::Chording => {
                    self.mouse_state = MouseState::DownUpAfterChording;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    return self.chording_click(pos.0, pos.1);
                }
                MouseState::ChordingNotFlag => {
                    self.mouse_state = MouseState::DownUpAfterChording;
                    self.right -= 1;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    return self.chording_click(pos.0, pos.1);
                }
                // 以下情况其实是不可能的
                MouseState::UpUp => {
                    if self.game_board[pos.0][pos.1] < 10 {
                        self.mouse_state = MouseState::UpDownNotFlag;
                    } else {
                        self.mouse_state = MouseState::UpDown;
                    }
                    return self.right_click(pos.0, pos.1);
                }
                MouseState::DownUp => self.mouse_state = MouseState::Chording,
                MouseState::DownUpAfterChording => self.mouse_state = MouseState::Chording,
                MouseState::Undefined => self.mouse_state = MouseState::UpUp,
            },
            "mv" => {}
            "mc" => {
                self.middle_hold = true;
            }
            "mr" => {
                self.middle_hold = false;
                return self.chording_click(pos.0, pos.1);
            }
            "m" => {
                self.middle_hold = !self.middle_hold;
                if !self.middle_hold {
                    return self.chording_click(pos.0, pos.1);
                }
            }
            "cc" => match self.mouse_state {
                MouseState::DownUp => self.mouse_state = MouseState::Chording,
                MouseState::DownUpAfterChording => self.mouse_state = MouseState::Chording,
                MouseState::UpDown => self.mouse_state = MouseState::Chording,
                MouseState::UpDownNotFlag => self.mouse_state = MouseState::ChordingNotFlag,
                _ => return Err(()),
            },
            "crl" => match self.mouse_state {
                MouseState::Chording => {
                    self.mouse_state = MouseState::UpDown;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    return self.chording_click(pos.0, pos.1);
                }
                MouseState::ChordingNotFlag => {
                    self.mouse_state = MouseState::UpDown;
                    self.right -= 1;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    return self.chording_click(pos.0, pos.1);
                }
                _ => return Err(()),
            },
            "crr" => match self.mouse_state {
                MouseState::Chording => {
                    self.mouse_state = MouseState::DownUpAfterChording;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    return self.chording_click(pos.0, pos.1);
                }
                MouseState::ChordingNotFlag => {
                    self.mouse_state = MouseState::DownUpAfterChording;
                    self.right -= 1;
                    if pos.0 == self.row && pos.1 == self.column {
                        return Ok(0);
                    }
                    return self.chording_click(pos.0, pos.1);
                }
                _ => return Err(()),
            },
            _ => return Err(()),
        }
        Ok(0)
    }
    /// 直接分析整局的操作流，中间也可以停顿
    /// 开始游戏前的任何操作也都记录次数
    pub fn step_flow(&mut self, operation: Vec<(&str, (usize, usize))>) -> Result<(), ()>
    where
        T: std::ops::Index<usize> + BoardSize + std::fmt::Debug,
        T::Output: std::ops::Index<usize, Output = i32>,
    {
        for op in operation {
            self.step(op.0, op.1)?;
        }
        Ok(())
    }
    fn is_win(&mut self) -> bool
    where
        T: std::ops::Index<usize> + BoardSize,
        T::Output: std::ops::Index<usize, Output = i32>,
    {
        for j in self.pointer_y..self.column {
            if self.game_board[self.pointer_x][j] < 10 {
                if self.game_board[self.pointer_x][j] != self.board[self.pointer_x][j] {
                    return false; // 安全性相关（发生作弊）
                }
            }
            if self.game_board[self.pointer_x][j] >= 10 && self.board[self.pointer_x][j] != -1 {
                self.pointer_y = j;
                return false;
            }
        }
        for i in self.pointer_x + 1..self.row {
            for j in 0..self.column {
                if self.game_board[i][j] < 10 {
                    if self.game_board[i][j] != self.board[i][j] {
                        return false; // 安全性相关（发生作弊）
                    }
                }
                if self.game_board[i][j] >= 10 && self.board[i][j] != -1 {
                    self.pointer_x = i;
                    self.pointer_y = j;
                    return false;
                }
            }
        }
        true
    }
    // 清空状态机里的点击次数
    fn clear_click_num(&mut self) {
        self.flag = 0;
        self.flagedList.clear();
        self.double = 0;
        self.left = 0;
        self.right = 0;
    }
}

/// 鼠标状态
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MouseState {
    UpUp,
    UpDown,
    /// 右键按下，且既没有标雷，也没有取消标雷的状态
    UpDownNotFlag,
    DownUp,
    /// 双键都按下的其他状态
    Chording,
    /// 双键都按下，且是在不可以右击的格子上、先按下右键
    ChordingNotFlag,
    /// 双击后先弹起右键，左键还没弹起的状态
    DownUpAfterChording,
    /// 未初始化的状态
    Undefined,
}

/// 游戏局面状态
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GameBoardState {
    Ready,
    /// 游戏开始，埋雷前标雷，将被记录到录像里。
    PreFlaging,
    Playing,
    Loss,
    Win,
    /// 局面作为录像在被播放
    Display,
}
