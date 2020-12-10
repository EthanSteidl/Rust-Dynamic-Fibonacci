
struct DynamicFibonacci
{
    table: Vec<usize>,
}
impl DynamicFibonacci
{
    fn new() -> DynamicFibonacci
    {
        let mut table = Vec::new();
        table.push(0);
        table.push(1);
        return DynamicFibonacci{ table:table};
    }
    fn fibonacci(&mut self, n: usize) -> usize
    {
        if n < self.table.len()
        {
            return self.table[n];
        }
        for i in self.table.len()..n+1
        {
            self.table.push(self.table[i-1] + self.table[i-2]);
        }
        return self.table[n];
    }
}

fn main() {
    println!("Hello, world!");
    let mut fib = DynamicFibonacci::new();
    let x = fib.fibonacci(17);
    println!("Result: {}", x);
    println!("Result: {}", fib.fibonacci(50))
}
