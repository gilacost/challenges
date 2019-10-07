use rand::{seq::SliceRandom, thread_rng};
use std::cmp::Ordering;
use std::env;

/*
  This module defines the functions to find a path in a 5x5 matrix.
  Expects an string that might have the next characters: [u,d,r,l,?].
  The letters stand for up, down, right and left.

  The question marks are directions in the coordinates that need to
  be found taking into account that the path needs to be unique and that
  should never go outside the 5X5 matrix.
  """

  ## Examples:

      iex> Challenges.Path.correct("???rrurdr?")
      "dddrrurdrd"

      iex> Challenges.Path.correct("drdr??rrddd?")
      "drdruurrdddd"

  ## First example matrix:

      d 0 0 0 0
      d 0 0 0 0
      d 0 r d 0
      r r u r d
      0 0 0 0 0

  ## Second example matrix:

      d 0 r r d
      r d u 0 d
      0 r u 0 d
      0 0 0 0 d
      0 0 0 0 0
*/

#[derive(PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_string = args[1].to_string();
    //TODO check that is string and contain only 'durl?'
    //TODO The first argument is the path that was used to call the program.
    println!("The path to correct is {}.", args[1]);
    correct_path(input_string);
}

fn correct_path(input_string: String) -> String {
    let mut path_corrected: String = "".to_string();
    let mut checked = false;

    println!("input_string, {:?}", input_string);

    while !checked {
        let mut test_string = String::new();
        let mut path = vec![Point { x: 0, y: 0 }];

        //itrate over string
        for c in input_string.chars() {
            let direction: char;

            if c == '?' {
                direction = random_direction();
            } else {
                direction = c;
            }

            test_string.push(direction);

            let &Point { x, y } = path.last().clone().unwrap();
            path.push(forward(direction, x, y));
            let &Point { x, y } = path.last().clone().unwrap();

            if x < 0 || x > 4 || y > 0 || y < -4 {
                break;
            }
        }

        println!("input_string, {:?}", input_string);

        println!("test_string, {:?}", test_string);

        let &Point { x, y } = path.last().clone().unwrap();

        // println!("corected path, {:?}", path);

        let test_path_len = path.len();

        path.sort_by(|a, b| compare_points(a, b));
        path.dedup();

        println!("unique path {:?}", path);

        if check_path(test_path_len, path.len(), x, y) {
            path_corrected = test_string;
            checked = true;
        }
    }
    return path_corrected;
}

fn check_path(test_path_len: usize, path_unique_len: usize, x: i32, y: i32) -> bool {
    if test_path_len == path_unique_len && x == 4 && y == -4 {
        return true;
    } else {
        return false;
    }
}

fn compare_points(a: &Point, b: &Point) -> Ordering {
    let &Point { x: x1, y: y1 } = a;
    let &Point { x: x2, y: y2 } = b;

    if x2 == x1 && y2 == y1 {
        return Ordering::Equal;
    } else if x2 > x1 {
        return Ordering::Greater;
    } else {
        return Ordering::Less;
    }
}

fn random_direction() -> char {
    let vs = vec!['d', 'u', 'l', 'r'];
    let direction = vs.choose(&mut thread_rng()).unwrap();
    return *direction;
}

fn forward(direction: char, x: i32, y: i32) -> Point {
    let position = match direction {
        'd' => Point { x: x, y: y - 1 },
        'r' => Point { x: x + 1, y: y },
        'u' => Point { x: x, y: y + 1 },
        'l' => Point { x: x - 1, y: y },
        _ => Point { x: x, y: y },
    };
    return position;
}

#[cfg(test)]
mod tests {
    use crate::correct_path;

    #[test]
    fn path_correction() {
        let path_to_correct = "???rrurdr?".to_string();
        let expected = "dddrrurdrd".to_string();
        assert_eq!(correct_path(path_to_correct), expected);

        let path_to_correct = "drdr??rrddd?".to_string();
        let expected = "drdruurrdddd".to_string();
        assert_eq!(correct_path(path_to_correct), expected);
    }
}
