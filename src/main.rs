pub mod helpers;
pub mod lexer;
pub mod parser;

use std::collections::VecDeque;

use lexer::lexer::lex;
use parser::parser::parse_program;

fn main() {
    // widget CounterPage(controller: CounterPageController)
    let input = "
<Button1[bg:yellow-100] @tap:controller.increment>
  <Text> \"Increment\"
<Button2[bg:red-100] @tap:controller.decrement>
  <Text> \"Decrement\"
<Container>
  <GlowingBox>
    <WavingAnimation>
      <Text> \"Happy hacking!\"
  <FittedBox>
    <Text> \"Counter: \" + controller.counter
      ";

    let input = "
<Button>
  <Text1>
  <Text2>
  <Text3>
  <Text4>
    ";

    let tokens = lex(input).unwrap();
    // println!("{:#?}", tokens);

    let ast = parse_program(&mut VecDeque::from(tokens)).unwrap();
    println!("================");
    println!("{:#?}", ast);
}
