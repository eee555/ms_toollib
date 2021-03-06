use crate::algorithms::{solve_direct, solve_enumerate, solve_minus};
use crate::board;
use crate::utils::{is_good_chording, refresh_matrix, refresh_matrixs};
use board::{AvfVideo, MouseState, VideoEvent};

// 录像的事件分析。参与分析的录像必须已经计算出对应的数据。
// error: 高风险的猜雷（猜对概率0.05）√
// feature: 高难度的判雷√
// warning: 可以判雷时视野的转移
// feature: 双线操作
// feature: 破空（成功率0.98）
// feature: 教科书式的FL局部(步数4)
// error: 过于弯曲的鼠标轨迹(500%)√
// warning：弯曲的鼠标轨迹(200%)√
// warning: 可以判雷时选择猜雷√
// warning: 没有作用的操作
// suspect: 点击速度过快(0.01)
// suspect: 鼠标移动过快(2)
// suspect: 笔直的鼠标轨迹(101%)√
pub fn analyse_high_risk_guess(video: &mut AvfVideo) {
    let mut x = (video.events[video.events.len() - 1].y / 16) as usize;
    let mut y = (video.events[video.events.len() - 1].x / 16) as usize;
    let mut id = video.events.len() - 1;
    for ide in (0..video.events.len() - 1).rev() {
        if video.events[ide].useful_level >= 2 {
            let p = video.events[ide].prior_game_board.get_poss()[x][y];
            if p >= 0.51 {
                video.events[id].comments = format!(
                    "{}{}",
                    video.events[id].comments,
                    format!("error: 危险的猜雷(猜对概率{:.3});", 1.0 - p)
                );
                // println!(
                //     "{:?} => {:?}",
                //     video.events[id].time, video.events[id].comments
                // );
            }
            x = (video.events[ide].y / 16) as usize;
            y = (video.events[ide].x / 16) as usize;
            id = ide;
        }
    }
}

pub fn analyse_jump_judge(video: &mut AvfVideo) {
    // 功能：检测左键的跳判
    let mut id_last = 0;
    loop {
        if video.events[id_last].mouse != "lr" {
            id_last += 1;
        } else {
            break;
        }
    }
    // let mut tb = video.events[id_last].posteriori_game_board.game_board_marked.clone();
    let mut x;
    let mut y;
    for ide in 0..video.events.len() {
        x = (video.events[ide].y / 16) as usize;
        y = (video.events[ide].x / 16) as usize;
        if video.events[ide].useful_level >= 2 && video.events[ide].mouse == "lr" {
            if !video.events[id_last]
                .prior_game_board
                .get_basic_not_mine()
                .contains(&(x, y))
                && video.events[id_last]
                    .prior_game_board
                    .get_enum_not_mine()
                    .contains(&(x, y))
            {
                video.events[ide].comments = format!(
                    "{}{}",
                    video.events[ide].comments,
                    format!("feature: 高难度的判雷(左键);")
                );
                // println!(
                //     "{:?} => {:?}",
                //     video.events[ide].time, video.events[ide].comments
                // );
            }
        } else if video.events[ide].useful_level == 1 && video.events[ide].mouse == "rc" {
            if !video.events[id_last]
                .prior_game_board
                .get_basic_is_mine()
                .contains(&(x, y))
                && video.events[id_last]
                    .prior_game_board
                    .get_enum_is_mine()
                    .contains(&(x, y))
            {
                video.events[ide].comments = format!(
                    "{}{}",
                    video.events[ide].comments,
                    format!("feature: 高难度的判雷(标雷);")
                );
                // println!(
                //     "{:?} => {:?}",
                //     video.events[ide].time, video.events[ide].comments
                // );
            }
        }

        if video.events[ide].useful_level >= 2 {
            id_last = ide;
            // tb = video.events[id_last].posteriori_game_board.clone();
        }
    }
}

pub fn analyse_needless_guess(video: &mut AvfVideo) {
    let mut id_last = 0;
    loop {
        if video.events[id_last].mouse != "lr" {
            id_last += 1;
        } else {
            break;
        }
    }
    // let mut tb = video.events[id_last].posteriori_game_board.clone();
    let mut x;
    let mut y;
    for ide in 0..video.events.len() {
        if video.events[ide].useful_level >= 2 && video.events[ide].mouse == "lr" {
            x = (video.events[ide].y / 16) as usize;
            y = (video.events[ide].x / 16) as usize;

            if video.events[id_last].prior_game_board.get_poss()[x][y] > 0.0
                && !video.events[id_last]
                    .prior_game_board
                    .get_basic_not_mine()
                    .contains(&(x, y))
                && !video.events[id_last]
                    .prior_game_board
                    .get_enum_not_mine()
                    .contains(&(x, y))
            {
                video.events[ide].comments = format!(
                    "{}{}",
                    video.events[ide].comments,
                    format!("warning: 可以判雷时选择猜雷;")
                );
                // println!(
                //     "{:?} => {:?}",
                //     video.events[ide].time, video.events[ide].comments
                // );
            }
        }
        if video.events[ide].useful_level >= 2 {
            id_last = ide;
            // tb = video.events[id_last].posteriori_game_board.clone();
        }
    }
}

pub fn analyse_mouse_trace(video: &mut AvfVideo) {
    let mut click_last = (video.events[0].x as f64, video.events[0].y as f64);
    let mut click_last_id = 0;
    let mut move_last = (video.events[0].x as f64, video.events[0].y as f64);
    let mut path = 0.0;
    for ide in 0..video.events.len() {
        let current_x = video.events[ide].x as f64;
        let current_y = video.events[ide].y as f64;
        path += ((move_last.0 - current_x).powf(2.0) + (move_last.1 - current_y).powf(2.0)).sqrt();
        move_last = (current_x, current_y);
        if video.events[ide].mouse == "lr"
            || video.events[ide].mouse == "rc"
            || video.events[ide].mouse == "rr"
        {
            let path_straight = ((click_last.0 - current_x).powf(2.0)
                + (click_last.1 - current_y).powf(2.0))
            .sqrt();
            let k = path / path_straight;
            if k > 7.0 {
                video.events[click_last_id].comments = format!(
                    "{}{}",
                    video.events[click_last_id].comments,
                    format!("error: 过于弯曲的鼠标轨迹({:.0}%);", k * 100.0)
                );
                // println!(
                //     "{:?} => {:?}",
                //     video.events[click_last_id].time, video.events[click_last_id].comments
                // );
            } else if k > 3.5 {
                video.events[click_last_id].comments = format!(
                    "{}{}",
                    video.events[click_last_id].comments,
                    format!("warning: 弯曲的鼠标轨迹({:.0}%);", k * 100.0)
                );
                // println!(
                //     "{:?} => {:?}",
                //     video.events[click_last_id].time, video.events[click_last_id].comments
                // );
            } else if k < 1.01 {
                if k > 5.0 {
                    video.events[click_last_id].comments = format!(
                        "{}{}",
                        video.events[click_last_id].comments,
                        format!("suspect: 笔直的鼠标轨迹;")
                    );
                    // println!(
                    //     "{:?} => {:?}",
                    //     video.events[click_last_id].time, video.events[click_last_id].comments
                    // );
                }
            }
            click_last = (video.events[ide].x as f64, video.events[ide].y as f64);
            click_last_id = ide;
            path = 0.0;
        }
    }
}

pub fn analyse_vision_transfer(video: &mut AvfVideo) {
    let mut click_last = (video.events[0].y as f64, video.events[0].x as f64);
    let mut l_x = (video.events[0].y / 16) as usize;
    let mut l_y = (video.events[0].x / 16) as usize;
    let mut click_last_id = 0;
    for ide in 0..video.events.len() {
        if video.events[ide].useful_level >= 2 {
            // let xx = (video.events[ide].y / 16) as usize;
            // let yy = (video.events[ide].x / 16) as usize;
            let click_current = (video.events[ide].y as f64, video.events[ide].x as f64);
            if ((click_last.0 - click_current.0).powf(2.0)
                + (click_last.1 - click_current.1).powf(2.0))
            .sqrt()
                >= 112.0
            {
                let mut flag = false;
                for &(xxx, yyy) in video.events[click_last_id]
                    .prior_game_board
                    .get_basic_not_mine()
                {
                    if xxx <= l_x + 3 && xxx + 3 >= l_x && yyy <= l_y + 3 && yyy + 3 >= l_y {
                        flag = true;
                    }
                }
                for &(xxx, yyy) in video.events[click_last_id]
                    .prior_game_board
                    .get_enum_not_mine()
                {
                    if xxx <= l_x + 3 && xxx + 3 >= l_x && yyy <= l_y + 3 && yyy + 3 >= l_y {
                        flag = true;
                    }
                }
                if flag {
                    video.events[click_last_id].comments = format!(
                        "{}{}",
                        video.events[click_last_id].comments,
                        format!("warning: 可以判雷时视野的转移;")
                    );
                    // println!(
                    //     "{:?} => {:?}",
                    //     video.events[click_last_id].time, video.events[click_last_id].comments
                    // );
                }
            }
            click_last = click_current;
            l_x = (video.events[ide].y / 16) as usize;
            l_y = (video.events[ide].x / 16) as usize;
            click_last_id = ide;
        }
    }
}

pub fn analyse_survive_poss(video: &mut AvfVideo) {
    // 计算扫开这局的后验开率
    let mut s_poss = 1.0;
    let mut message = "luck: ".to_string();
    let mut click_last_id = 0;
    let mut has_begin = false;
    for ide in 0..video.events.len() {
        if video.events[ide].mouse == "lr" && video.events[ide].useful_level > 0 {
            if !has_begin {
                has_begin = true;
                continue;
            }
            let l_x = (video.events[ide].y / 16) as usize;
            let l_y = (video.events[ide].x / 16) as usize;
            let p = video.events[click_last_id].prior_game_board.get_poss()[l_x][l_y];
            if p > 0.0 && p < 1.0 {
                s_poss *= 1.0 - p;
                message.push_str(&format!("{:.3} * ", 1.0 - p));
                // println!("{:?} ==> {:?}", video.events[ide].time, 1.0 - p);
            }
            click_last_id = ide;
        }
    }
    if message.len() > 7 {
        message.pop();
        message.pop();
        message.push_str("= ");
        message.push_str(&format!("{:.6};", s_poss));
    } else {
        message.push_str("1;");
    }
    video.events.last_mut().unwrap().comments = message;
}

#[derive(Debug, PartialEq)]
pub enum SuperFLState {
    NotStart,   // 还没开始
    StartNow,   // 此处开始
    StartNotOk, // 开始了，但还没满足数量
    IsOk,       // 满足数量了，延续
    Finish,     // 检测到，结束
}
pub fn analyse_super_fl_local(video: &mut AvfVideo) {
    let event_min_num = 5;
    let euclidean_distance = 16;
    let mut anchor = 0;
    let mut counter = 0; //正在标雷、双击超过event_min_num总次数
    let mut state = SuperFLState::NotStart;
    let mut last_rc_num = 0; // 最后有几个右键
    let mut last_ide = 0;
    for ide in 1..video.events.len() {
        if video.events[ide].mouse == "mv" {
            continue;
        }
        let x = video.events[ide].y as usize / 16;
        let y = video.events[ide].x as usize / 16;
        let x_1 = video.events[last_ide].y as usize / 16;
        let y_1 = video.events[last_ide].x as usize / 16;
        // if video.events[ide].mouse == "lr" || video.events[ide].mouse == "rr"{
        //     println!("{:?}+++{:?}", video.events[last_ide].time, video.events[last_ide].mouse_state);
        //     // println!("---{:?}", video.events[ide].useful_level);
        // }

        if video.events[ide].mouse == "rc"
            && video.events[ide].prior_game_board.game_board[x][y] == 10
            && video.events[ide].useful_level == 1
        {
            // 正确的标雷
            match state {
                SuperFLState::NotStart => {
                    state = SuperFLState::StartNow;
                    counter = 1;
                    last_rc_num = 1;
                    anchor = ide;
                    // println!("666");
                }
                SuperFLState::StartNow => {
                    state = SuperFLState::StartNotOk;
                    counter += 1;
                    last_rc_num += 1;
                }
                SuperFLState::StartNotOk | SuperFLState::IsOk => {
                    counter += 1;
                    last_rc_num += 1;
                }
                _ => {}
            }
        } else if video.events[ide].useful_level == 3 {
            // 正确的双击
            if !is_good_chording(&video.events[ide].prior_game_board.game_board, (x, y)) {
                match state {
                    SuperFLState::IsOk => {
                        counter -= last_rc_num;
                        state = SuperFLState::Finish;
                    }
                    _ => {
                        state = SuperFLState::NotStart;
                        counter = 0;
                        last_rc_num = 0;
                    }
                }
            } else {
                match state {
                    SuperFLState::StartNow => {
                        state = SuperFLState::StartNotOk;
                        counter += 1;
                        last_rc_num = 0;
                    }
                    SuperFLState::StartNotOk | SuperFLState::IsOk => {
                        counter += 1;
                        last_rc_num = 0;
                    }
                    _ => {}
                }
            }
        } else if video.events[ide].mouse == "lr"
            && (video.events[last_ide].mouse_state == MouseState::DownUp
                || video.events[last_ide].mouse_state == MouseState::Chording)
            || video.events[ide].mouse == "rr"
                && video.events[last_ide].mouse_state == MouseState::Chording
        {
            // 左键或错误的右键或错误的双键
            match state {
                SuperFLState::IsOk => {
                    counter -= last_rc_num;
                    state = SuperFLState::Finish;
                }
                _ => {
                    state = SuperFLState::NotStart;
                    counter = 0;
                    last_rc_num = 0;
                }
            }
        }
        if (x as i32 - x_1 as i32) * (x as i32 - x_1 as i32)
            + (y as i32 - y_1 as i32) * (y as i32 - y_1 as i32)
            > euclidean_distance
        {
            match state {
                SuperFLState::StartNotOk => {
                    state = SuperFLState::NotStart;
                    counter = 0;
                    last_rc_num = 0;
                }
                SuperFLState::IsOk => {
                    counter -= last_rc_num;
                    state = SuperFLState::Finish;
                }
                _ => {}
            }
        }
        if counter - last_rc_num >= event_min_num {
            match state {
                SuperFLState::StartNow | SuperFLState::StartNotOk => {
                    state = SuperFLState::IsOk;
                }
                _ => {}
            }
        }
        match state {
            SuperFLState::Finish => {
                video.events[anchor].comments = format!(
                    "{}{}",
                    video.events[anchor].comments,
                    format!("feature: 教科书式的FL局部(步数{});", counter)
                );
                state = SuperFLState::NotStart;
            }
            _ => {}
        }
        last_ide = ide;
        // println!("{:?}", video.events[last_ide].mouse_state);
    }
}
