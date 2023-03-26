# Faker

Generate fake data for testing and development.

Built with Rust and compiled to WebAssembly.

Only first name, last name, and full name are supported right now. More data
types are planned.

First names are a combination of the top 50 male and top 50 female first names
given to babies born in the USA during 1922-2021 according to the [Social
Security
Administration](https://www.ssa.gov/OACT/babynames/decades/century.html).

Last names are the top 100 surnames in the USA according to the 2020 Census as
reported by
[ThoughCo.](https://www.thoughtco.com/most-common-us-surnames-1422656)

Full names are a concatenation of first name and last name with a space.

## Usage

I suggest using [Wasmer](https://wasmer.io/) to run this code on anything that
can use WebAssembly. [This code is published on
wapm.](https://wapm.io/travishorn/faker)

### Example: JavaScript

Note: Make sure you have installed Node.js and wapm (comes bundled with Wasmer).

Make a new directory for your project

```sh
mkdir faker-demo
```

Change into the directory

```sh
cd faker-demo
```

Initialize a new npm package

```sh
npm init -y
```

Edit `package.json` and add this line to make the package a module

```json
"type": "module",
```

Install the library

```sh
wapm install travishorn/faker --npm
```

Create a new file called `index.js` with the following content

```javascript
import { bindings } from "@travishorn/faker";
const faker = await bindings.faker();
console.log(faker.fullName());
```

Run the file with Node.js

```sh
node .
```

A random full name is output to the terminal

```sh
Janet Price
```
