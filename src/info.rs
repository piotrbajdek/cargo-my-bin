#![warn(clippy::nursery, clippy::pedantic)]

pub fn help(reset: &str, blue_underlined: &str, cyan: &str, grey: &str, yellow: &str) {
    println!("The plugin cargo-my-bin displays the contents of {}", blue_underlined.to_owned() + "~/.cargo/bin/" + reset + ".");
    println!();
    println!("{}", grey.to_owned() + "Usage" + reset + ":     " + yellow + "cargo my-bin [empty / options]" + reset);
    println!();
    println!("{}", grey.to_owned() + "Options" + reset + ":   " + cyan + "--default" + reset + "       Display crates installed by default" + cyan);
    println!("           --plugins{}", reset.to_owned() + "       Display installed Cargo plugins only");
    println!();
    println!("{}", grey.to_owned() + "Examples" + reset + ":  " + yellow + "cargo my-bin" + reset + "    [display all files in " + blue_underlined + "~/.cargo/bin/" + reset + "]" + yellow);
    println!("           cargo my-bin --plugins{}", reset.to_owned() + "  [only display Cargo plugins]");
    println!();
    println!("{}", grey.to_owned() + "See also" + reset + ":  " + cyan + "--help" + reset + "          Show this help");
    println!("           {}", cyan.to_owned() + "--license" + reset + "       Show licensing information");
    println!("           {}", cyan.to_owned() + "--version" + reset + "       Show the program version");
}

pub fn version(reset: &str, grey: &str, yellow: &str) {
    println!("{}", grey.to_owned() + "Version" + reset + ": " + yellow + "0.2.0" + reset);
    println!("January 22, 2023");
}

pub fn license(reset: &str, yellow: &str) {
    println!("{}", yellow.to_owned() + "Copyright © 2022–2023 Piotr Bajdek" + reset);
    println!();
    println!("The Universal Permissive License (UPL), Version 1.0");
    println!();
    println!("Subject to the condition set forth below, permission is hereby granted to any");
    println!("person obtaining a copy of this software, associated documentation and/or data");
    println!(r#"(collectively the "Software"), free of charge and under any and all copyright"#);
    println!("rights in the Software, and any and all patent rights owned or freely");
    println!("licensable by each licensor hereunder covering either (i) the unmodified");
    println!("Software as contributed to or provided by such licensor, or (ii) the Larger");
    println!("Works (as defined below), to deal in both");
    println!();
    println!("(a) the Software, and");
    println!();
    println!("(b) any piece of software and/or hardware listed in the lrgrwrks.txt file if");
    println!("one is included with the Software (each a “Larger Work” to which the Software");
    println!("is contributed by such licensors),");
    println!();
    println!("without restriction, including without limitation the rights to copy, create");
    println!("derivative works of, display, perform, and distribute the Software and make,");
    println!("use, sell, offer for sale, import, export, have made, and have sold the");
    println!("Software and the Larger Work(s), and to sublicense the foregoing rights on");
    println!("either these or other terms.");
    println!();
    println!("This license is subject to the following condition:");
    println!();
    println!("The above copyright notice and either this complete permission notice or at");
    println!("a minimum a reference to the UPL must be included in all copies or");
    println!("substantial portions of the Software.");
    println!();
    println!(r#"THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR"#);
    println!("IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,");
    println!("FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE");
    println!("AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER");
    println!("LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,");
    println!("OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE");
    println!("SOFTWARE.");
}
