<h2><img src="./images/logo.png" alt="Favicon Preview" width="100" align="center"> react-comps-generator</h2>

[![Repo](https://img.shields.io/badge/alchemmist%2Freact--comps--generator-blue?logo=github&label=github&color=blue)](https://github.com/alchemmist/react-comps-generator)
![GitHub Release](https://img.shields.io/github/v/release/alchemmist/react-comps-generator)
![Last commit](https://img.shields.io/github/last-commit/alchemmist/react-comps-generator?style=flat)
![Stars](https://img.shields.io/github/stars/alchemmist/react-comps-generator?style=flat)
![Forks](https://img.shields.io/github/forks/alchemmist/react-comps-generator?style=flat)
![License](https://img.shields.io/github/license/alchemmist/react-comps-generator?style=flat)
![Contributors](https://img.shields.io/github/contributors/alchemmist/react-comps-generator?style=flat)
![Rust](https://img.shields.io/badge/1.85-default?label=rust)
![Colored](https://img.shields.io/badge/2.04-default?label=colored)

**About:** Blazingly fast command line utility for generating React components faster.

**Motivation:** Imagine you're writing on React. You need to create new component, or new page, or new module, etc. What you need to do is:
1. go to directory
2. make directory like `your-component`
3. in this directory create two empty files: `YourComponent.jsx` and `YourComponent.css`
4. in the `.jsx` file you need to write the basic skeleton:
```jsx
import "./YourComponent.css";

const YourComponent = () => {
    return (
        <>
            <some elements>
        </>
    );
}

export default YourComponent;
```
And then finally, only then can you write a business logic. In this pipeline at every stop you can make a type or a mistake. Then you have to spend time looking through all the titles. Culmination: 👎

### Usage
With this utility pipeline the pipeline becomes a single command to start writing business logic:
```sh
rgc # if you have useful alias
```
Then you need to select what you want to create:
```
Which folder should we put the component in?
c — components, p — pages, m - modules:
```
Set the name:
```
What do we call the component? pages/
```
Accept changes:
```
So I'm creating the files:

        ./pages/Test.jsx
        ./pages/Test.scss

Ok? [y]/n:
```
Done!

### Demo
<img src="./images/demo.gif" alt="Demo GIF" width="650">

### Installation
After cloning repo run:
```bash
cd react-comps-generator
cargo build --release
```
Then add to `$PATH` this directory:
```
/path/to/repo/react-comps-generator/target/release
```
To use shorter `rgc` command you can add alias to your shell:
```zsh
alias rgc="react-comps-generator"
```
Or make alias to `release` folder, to avoid editing `$PATH`.

### Questions | Contribute

If you have questions about this repo or you want to contribute, [text me](https://t.me/alchemmist)!

### License
Licensed under of The [MIT](./LICENSE) License.

### Contributors
- [@alchemmist](https://github.com/alchemmist) as Anton Grishin

