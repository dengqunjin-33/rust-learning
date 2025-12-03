struct Struct{
    e:i32
}
fn main() {
    println!("{} days",31);
    println!("{subject} {verb} {object}",object = "xxx",subject = "yyy",verb = "zzz");
    println!("!base 10         {}",69240);
    println!("!base 2(bin)         {:b}",69240);
    println!("!base 8(oct)         {:o}",69240);
    println!("!base 16(hex)         {:x}",69240);
    //------------------
    println!("{number:0>5}",number=1);
    println!("{number:0<5}",number=1);
    println!("{number:0>width$}",number=1,width=5);
    println!("myname is {0},{1},{0}","xxx","yyy");
    // struct Structure(i32);
    let number:f64 = 1.0;
    let width:usize = 5;
    println!("{number:>width$}");

    let mut x= 5;
    println!("the value is :{}",x);
    x = 6;
    println!("the value is :{}",x);

    let _x=5;
    //warning
    let y = 10;

    let (a,mut b):(bool,bool) = (true,false);
    println!("a = {:?},b = {:?}",a,b);
    // b=true;
    // a=false;
    b=true;
    assert_eq!(a,b,"xxxx");

    let (a,b,c,d,e);
    (a,b) = (1,2);
    [c,..,d,_] = [1,2,3,4,5];
    Struct {e,..} = Struct{e:5};

    assert_eq!([1,2,1,4,5],[a,b,c,d,e],"ssss");

    const MAX_POINT:u32 = 100_000;

    //变量遮蔽
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("1.current value is :{}",x);
    }
    println!("2.current value is :{}",x);

    let spaces = "     ";
    //这里会重新赋值
    let spaces = spaces.len();
    

    let mut spaces = "     ";
    //这里不会重新赋值 报错expected `&str`, found `usize`
    spaces = spaces.len();

}
