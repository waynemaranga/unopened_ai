# 😐 UNOPENED AI

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white) ![Gemini](https://img.shields.io/badge/Gemini-8E75B2?style=for-the-badge&logo=googlebard&logoColor=fff) ![OpenAi](https://img.shields.io/badge/ChatGPT-74aa9c?style=for-the-badge&logo=openai&logoColor=white)

A terminal-based chatbot written in Rust using Gemini & OpenAI, and the `ratatui` crate for the front-end,

_Requires:_

1. Rust 2021 or greater <https://rustup.rs/>
   or run `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Gemini Pro API key <https://aistudio.google.com/app/apikey>
3. OpenAI API key
   _To start:_

```bash
git clone https://github.com/waynemaranga/unopened_ai.git
cd unopened_ai
cp -v .env.example .env
```

Add your Google API Key to the `.env` file

_...then build the project to install dependencies:_

```bash
cargo build -j 4 # install dependencies
cargo run # run the app
```

©️ 2024, Wayne Maranga

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at [LICENSE](/LICENSE.md)

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
