# Guessing Game

This is a simple guessing game in Rust. The program generates a random number between 1 and 100 and asks the user to guess the number. 

## How to Play

1. The program displays the message "Guess the number!" and then generates a random number between 1 and 100.
2. The program then asks the user to input their guess.
3. The program checks if the user's guess is a valid number between 1 and 100. If the guess is not valid, the program asks the user to input their guess again.
4. The program compares the user's guess with the secret number. If the guess is less than the secret number, the program displays the message "Too small!". If the guess is greater than the secret number, the program displays the message "Too big!". If the guess is equal to the secret number, the program displays the message "You win!" and the game ends.
5. The program repeats steps 2-4 until the user guesses the correct number.

## How to Run the Program

To run the program, you need to have Rust installed on your computer. You can download Rust from the official website: https://www.rust-lang.org/tools/install

Once you have Rust installed, you can download the source code for this program and run it from the command line.

1. Clone the repository to your local machine using the following command:
git clone https://github.com/Wamitinewton/rustaceans.git

2. migrate to directory containing the guessing game using the following command:
cd rustaceans/guessing_game

3. build the guessing game using the following command:
cargo build

3. Run the program using the following command:
cargo run

## Contributing

Contributions are welcome! If you have any ideas for improving this program or want to contribute code, please open an issue or submit a pull request.
