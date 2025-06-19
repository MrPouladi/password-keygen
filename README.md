# Password Keygen

A lightweight Rust tool that runs in the background and listens for a hotkey (default: Numpad 8). When triggered, it generates a secure 24-character password and copies it to the clipboard for quick pasting.

## Features

- Global hotkey listener using `rdev`
- Secure password generation with letters, numbers, and symbols
- Clipboard integration using `copypasta`
- Runs silently in the background

## How to Use (Windows)

1. Install Rust (if not installed):  
   https://www.rust-lang.org/tools/install

2. Clone or download this repository:
   - Open Command Prompt or PowerShell
   - Navigate to the project directory

3. Build the project:
   ```
   cargo build --release
   ```

4. Run the executable:
   ```
   .\target\release\password-keygen.exe
   ```

5. Press **Numpad 8** to generate a password and automatically copy it to your clipboard.

## Configuration

- Password length is fixed at 24 characters
- Charset includes:
  - Uppercase A–Z
  - Lowercase a–z
  - Digits 0–9
  - Symbols like `!@#$%^&*()_+-=[]{}|;:',.<>?/`

You can customize the charset or keybind in the source code.

## Dependencies

- [rand](https://crates.io/crates/rand)
- [rdev](https://crates.io/crates/rdev)
- [copypasta](https://crates.io/crates/copypasta)

## License

MIT License. See `LICENSE` for full details.
