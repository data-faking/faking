use crate::utils::seeder;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = zh_animal_cat)]
pub fn cat() -> String {
	ZH_CN__CAT[seeder::gen_range(0..ZH_CN__CAT_LEN)].to_string()
}

static ZH_CN__CAT: [&'static str; 20] = [
  "中华田园猫",
  "中国狸花猫",
  "山东狮子猫",
  "玄猫",
  "黑白花猫",
  "三花猫",
  "玳瑁猫",
  "橘猫",
  "四川简州猫",
  "中国大白猫",
  "美国短毛猫",
  "英国短毛猫",
  "加菲猫",
  "波斯猫",
  "布偶猫",
  "苏格兰折耳猫",
  "暹罗猫",
  "斯芬克斯猫",
  "德文卷毛猫",
  "阿比西尼亚猫",
];
static ZH_CN__CAT_LEN: usize = ZH_CN__CAT.len();