use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub fluimte);

use fluimte::WidgetParser;

fn main() {
    let input = "\
widget CounterPage(controller: CounterPageController)
  <Button1[bg:green-100] @tap:controller.increment>
    <Text> \"Increment\"
  <Button2[bg:red-100] @tap:controller.decrement>
    <Text> \"Decrement\"
  <Container>
    <GlowingBox>
      <WavingAnimation> 
        <Text> Happy hacking!
    <FittedBox>
      <Text> \"Counter: \" + controller.counter
  ";
}
