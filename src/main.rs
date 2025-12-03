fn main(){
    //报错type must be known at this point
    //let guess = "42".parse().expect("not a number");

    //a的值是255 由于+20溢出了 导致b=255+20-256=19
    // let a:u8 = 255;
    // let b = a.wrapping_add(20);
    // println!("a value is {} b value is {}",a,b);

    // let x = 2.0;//默认是f64
    // let y:f32 = 3.0;

    //这里由于精度丢失会报错 
    // assert!(0.1 + 0.2 == 0.3);

    // let abc:(f32,f32,f32) = (0.1,0.2,0.3);
    // let xyz:(f64,f64,f64) = (0.1,0.2,0.3);
    // println!("abc");
    // //0.1+0.2:3e99999a
    // println!("  0.1+0.2:{:x}",(abc.0 + abc.1).to_bits());
    // //0.3:3e99999a
    // println!("      0.3:{:x}",(abc.2).to_bits());
    // println!("xyz");
    // //0.1+0.2:3fd3333333333334
    // println!("  0.1+0.2:{:x}",(xyz.0 + xyz.1).to_bits());
    // //0.3:3fd3333333333333
    // println!("      0.3:{:x}",(xyz.2).to_bits());
    // //这里精度比较低所以相等
    // assert!(abc.0 + abc.1 == abc.2);
    // //这里精度太高丢失精度导致不相等报错
    // assert!(xyz.0 + xyz.1 == xyz.2);

    // let x = (-42.0_f32).sqrt();
    // if x.is_nan(){
    //     println!("未定义的数学行为")
    // }

    // //加法
    // let sum = 5 + 10;
    // //减法
    // let diff = 95.5 - 4.3;
    // //乘法
    // let produce = 4 * 30;
    // //除法
    // let quotient = 56.7/32.0;
    // 这一行报错 因为56.7类型是f64,32类型是i32。所以报错
    // let quotient = 56.7/32;
    // //余数
    // let remainder = 43 % 5;
    // //输出 15 91.2 120 1.771875 3
    // println!("{} {} {} {} {}",sum,diff,produce,quotient,remainder);

    
}