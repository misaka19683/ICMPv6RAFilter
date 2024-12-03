use once_cell::sync::Lazy;
//use std::net::Ipv6Addr;
use std::sync::Mutex;
use ipnet::Ipv6Net;

//use crate::prefix_info;

// 定义全局变量，存储 Ipv6Addr 类型的数据,用 Lazy + Mutex 包裹以实现线程安全
pub static GLOBAL_CONTAINER: Lazy<Mutex<Vec<Ipv6Net>>> = Lazy::new(|| Mutex::new(Vec::new()));

/// 添加数据到全局变量
pub fn add_to_container(ip: Ipv6Net) {
    let mut container = GLOBAL_CONTAINER.lock().unwrap();
          // ip.addr()&ip.netmask()
    container.push(ip.trunc());
    //Returns a copy of the network with the address truncated to the prefix length.
}

/// 获取全局变量中的数据
pub fn get_container_data() -> Vec<Ipv6Net> {
    let container = GLOBAL_CONTAINER.lock().unwrap();
    //container.clone().iter().map(|x| x.addr()).collect()
    container.clone()
}

pub static QUEUE_NUM: u16 = 0; // 队列号全局变量
