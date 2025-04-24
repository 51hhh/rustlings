mod sausage_factory {
    // 私有函数，仅模块内部可见
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 将make_sausage标记为pub，使其对外可见
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}