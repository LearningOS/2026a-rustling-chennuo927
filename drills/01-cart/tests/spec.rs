//! 购物车结算器的行为规格。
//!
//! 这些测试就是题目的精确版本：先把它从头到尾读一遍，
//! 弄清楚每个函数的**参数类型**和**返回类型**，再动手写。
//! 不要先去改测试 —— 测试是判卷人。

use cart::{apply_discount, cheapest, subtotal, tax_cents, total, Item};

fn book(title: &str, price_cents: u32) -> Item {
    Item::Book {
        title: title.to_string(),
        price_cents,
    }
}

fn food(name: &str, price_cents: u32) -> Item {
    Item::Food {
        name: name.to_string(),
        price_cents,
    }
}

fn toy(name: &str, price_cents: u32) -> Item {
    Item::Toy {
        name: name.to_string(),
        price_cents,
    }
}

fn sample() -> Vec<Item> {
    vec![
        book("Rust 程序设计语言", 4500),
        food("苹果", 1200),
        toy("魔方", 1000),
    ]
}

#[test]
fn empty_cart_costs_nothing() {
    let items: Vec<Item> = Vec::new();
    assert_eq!(subtotal(&items), 0);
    assert_eq!(total(&items), 0);
}

#[test]
fn subtotal_adds_every_item() {
    assert_eq!(subtotal(&sample()), 4500 + 1200 + 1000);
}

#[test]
fn books_and_food_are_tax_free() {
    assert_eq!(tax_cents(&book("Rust 程序设计语言", 4500)), 0);
    assert_eq!(tax_cents(&food("苹果", 1200)), 0);
}

#[test]
fn toys_are_taxed_at_13_percent_rounded_to_the_nearest_cent() {
    assert_eq!(tax_cents(&toy("魔方", 1000)), 130);
    assert_eq!(tax_cents(&toy("拼图", 33)), 4); // 4.29 -> 4
    assert_eq!(tax_cents(&toy("积木", 50)), 7); // 6.50 -> 7
}

#[test]
fn total_is_subtotal_plus_the_tax_of_every_item() {
    assert_eq!(total(&sample()), 4500 + 1200 + 1000 + 130);
}

#[test]
fn cheapest_of_an_empty_cart_is_none() {
    assert!(cheapest(&[]).is_none());
}

#[test]
fn cheapest_returns_the_first_item_when_prices_are_equal() {
    let items = vec![food("苹果", 1000), toy("魔方", 1000)];
    match cheapest(&items) {
        Some(Item::Food { name, .. }) => assert_eq!(name, "苹果"),
        other => panic!("期望拿到并列最便宜的那个商品，实际是 {:?}", other),
    }
}

#[test]
fn discount_rounds_down_to_the_cent() {
    let mut items = vec![book("Rust 程序设计语言", 4500), toy("魔方", 1001)];
    assert!(apply_discount(&mut items, 50).is_ok());
    assert_eq!(subtotal(&items), 2250 + 500);
}

#[test]
fn a_hundred_percent_discount_makes_everything_free() {
    let mut items = sample();
    assert!(apply_discount(&mut items, 100).is_ok());
    assert_eq!(subtotal(&items), 0);
}

#[test]
fn an_invalid_discount_is_rejected_and_changes_nothing() {
    let mut items = sample();
    let before = subtotal(&items);
    match apply_discount(&mut items, 101) {
        Err(_) => {}
        Ok(()) => panic!("101% 的折扣应该被拒绝"),
    }
    assert_eq!(subtotal(&items), before);
}
