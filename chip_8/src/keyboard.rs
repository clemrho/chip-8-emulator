// pub struct Keyboard{
//     input_char : char,
// }

// impl Keyboard {
//     /*  transfer modern keyboard map below:
//             1   2   3   4
//             Q   W   E   R
//             A   S   D   F
//             Z   X   C   V
//      to the chip-8 keyboard map:
//             1   2   3   C
//             4   5   6   D
//             7   8   9   E
//             A   0   B   F
//     clicking all other keys will cause it to enter '0'.
//      */
//     pub fn char_injection(self) -> char {
//         let output: char = match self.input_char {
//             '1' => '1',
//             '2' => '2',
//             '3' => '3',
//             '4' => 'C',
//             'Q' => '4',
//             'W' => '5',
//             'E' => '6',
//             'R' => 'D',
//             'A' => '7',
//             'S' => '8',
//             'D' => '9',
//             'F' => 'E',
//             'Z' => 'A',
//             'X' => '0',
//             'C' => 'B',
//             'V' => 'F',
//             _ => '0',
//         };
//     output
//     }
// }