#![macro_use]

macro_rules! arith_impl {
    ($self:ident, $name:literal, $op:tt) => {
        {
            let rhs = $self.stack.pop().unwrap();
            let lhs = $self.stack.pop().unwrap();

            match (lhs, rhs) {
                (Value::Number(a), Value::Number(b)) => $self.stack.push(Value::Number(a $op b)),
                (other_a, other_b) => panic!("Cannot {} {} and {}", $name, other_a, other_b),
            }
        }
    }
}
