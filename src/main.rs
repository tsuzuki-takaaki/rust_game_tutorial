use ncurses::mvprintw;

fn main() {
  let a = mvprintw(2, 3, "hello");
  println!("{a}");
}
