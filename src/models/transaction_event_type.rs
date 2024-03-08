/*
金额流向 发起方 --> 接收方
*/
// # 支付事件
const TE_TYPE_PAYMENT: &str = "payment";
// # 转账事件
const TE_TYPE_TRANSFER: &str = "transfer";
// # 发放奖励
const TE_TYPE_DISTRIBUTE: &str = "distribute";

// # 金额流向 发起方 <-- 接收方
// # 正常扣款
const TE_TYPE_DEDUCTION: &str = "deduction";

// # 金额回收
pub const TE_TYPE_RECOVERY: &str = "recovery";

// # 罚款
const TE_TYPE_FINE: &str = "fine";

// # 金额流向 接收方 --> 发起方 --> 接收方
// # 带金额回收的奖励发放（复合操作）
const TE_TYPE_DISTRIBUTE_WITH_RECOVERY: &str = "distribute_with_recovery";

// # 【支付】类型事件
const ALL_PAYMENT_TYPES: [&str; 3] = [TE_TYPE_PAYMENT, TE_TYPE_TRANSFER, TE_TYPE_DISTRIBUTE];

// # 【收入】类型事件
const ALL_INCOME_TYPES: [&str; 3] = [TE_TYPE_DEDUCTION, TE_TYPE_RECOVERY, TE_TYPE_FINE];

// # 【复合】类型事件
const ALL_COMPLEX_TYPES: [&str; 1] = [TE_TYPE_DISTRIBUTE_WITH_RECOVERY];

// # 所有事件
const ALL_TYPES: [&str; 7] = [
    TE_TYPE_PAYMENT,
    TE_TYPE_TRANSFER,
    TE_TYPE_DISTRIBUTE,
    TE_TYPE_DEDUCTION,
    TE_TYPE_RECOVERY,
    TE_TYPE_FINE,
    TE_TYPE_DISTRIBUTE_WITH_RECOVERY,
];

// 支出
pub const PAYMENT: i8 = -1;
// 收入
pub const INCOME: i8 = 1;
// 复合操作 双向都有
pub const COMPLEX: i8 = 0;

pub fn is_complex_event(event: &str) -> bool {
    return ALL_COMPLEX_TYPES.contains(&event);
}

pub fn split_complex_event(event: &str) -> Vec<&str> {
    if is_complex_event(event) {
        if event == TE_TYPE_DISTRIBUTE_WITH_RECOVERY {
            return vec![TE_TYPE_RECOVERY, TE_TYPE_DISTRIBUTE];
        }
    }
    return vec![event];
}

pub fn get_direction(event: &str) -> i8 {
    if ALL_PAYMENT_TYPES.contains(&event) {
        return PAYMENT;
    } else if ALL_INCOME_TYPES.contains(&event) {
        return INCOME;
    } else if ALL_COMPLEX_TYPES.contains(&event) {
        return COMPLEX;
    } else {
        panic!("未知事件类型: {}", event);
    }
}

pub fn check_event_type(event: &str) {
    if !ALL_TYPES.contains(&event) {
        panic!("未知事件类型: {}", event);
    }
}

pub fn get_opposite_direction(direction: i8) -> i8 {
    if direction == PAYMENT {
        return INCOME;
    } else {
        return PAYMENT;
    }
}
