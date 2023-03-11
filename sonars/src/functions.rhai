fn fuzz(func, d, num, t) {
    let res = 0.0;
    for i in 0..num{
        res += func(t+d*num);
        res += func(t-d*num);
    }
    res/(num*2.0)
}

fn test_func(){
    1.0
}