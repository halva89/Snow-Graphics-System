# Snow Graphics System

<!-- Badges -->
[![License](https://img.shields.io/badge/License-BSL%201.1-orange)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange)](https://www.rust-lang.org/)
[![Vulkan](https://img.shields.io/badge/Vulkan-1.0%2B-red)](https://www.vulkan.org/)
[![Windows](https://img.shields.io/badge/Windows-10%2B-blue)]([![Windows](https://img.shields.io/badge/Windows-10%2B-blue)](https://www.microsoft.com/windows))
[![Version](https://img.shields.io/badge/Version-Lithium%20v2.1-blueviolet)](https://github.com/halva89/Snow-Graphics-System/releases)
[![Author](https://img.shields.io/badge/Author-Halva-blue)](https://github.com/halva89)

---
## RU
### Snow Graphics System(далее SGS) - бесплатный open-source графический движок написанный на `Rust` с использованием `Vulkan`. ***ЗАПРЕЩЕНО КОММЕРЧЕСКОЕ ИСПОЛЬЗОВАНИЕ***
---
### Версии обозначаются элементами таблицы Менделеева. Просмотреть все релизы можно [здесь](https://github.com/halva89/Snow-Graphics-System/releases)

1. Alpha 1: Uranium. Создан первый треугольник и шейдеры. Архитектура не обозначена. [X] ВЫПОЛНЕНО
2. Alpha 2: Gold. Создан базовый парсер файлов `.sgss` формата. Архитектура Rusty Wolf [X] ВЫПОЛНЕНО
3. Alpha 3.0: Lithium. Готов полноценный парсер 2D фигур(треугольник, круг, квадрат). В комплект релиза входит демо-сцена. Архитектура Red Wolf [X] ВЫПОЛНЕНО
4. Alpha 3.1: Lithium V2. Готовы анимации(alpha, bugs included). Полноценный парсер `.sgss`, включая `animation` и остальные фигуры, а также настройки окна. Добавлена камера с калибровкой по вьюпорту. Выход с уничтожением ресурсов [X] ВЫПОЛНЕНО
5. Pre-beta: Beryllium. Устранение багов, базовые наброски 3D-фигур. Многопоточный рендеринг и рефакторинг. [/] В РАЗРАБОТКЕ
6. Beta 1: Radium. Создание теней, полноценная камера. [|] В ПОДГОТОВКЕ
7. Версии будут пополнятся
---
### Системные требования(последняя версия)
### Минимальные:
Система: Windows 10
Процессор: Core i3 / Ryzen 3
Оперативная память: 4ГБ
Видеокарта: с поддержкой Vulkan 1.0(младше 2015 года)

### Рекомендуемые:
Система: Windows 10/11
Процессор: Core i5 / Ryzen 5
Оперативная память: 8ГБ
Видеокарта: GeForce GTX / Radeon RX 5xx
---
### Установка
1. Установите из [релизов](https://github.com/halva89/Snow-Graphics-System/releases) последнюю, подходящую или желаемую версию
2. Распакуйте папку `release`.
3. Откройте папку и файл `snow_graphics.exe`
   
*ИЛИ*

1. Установите `source` последней версии и Rust SDK
2. Перейдите в папку установленных исходников.
3. Пропишите ```cargo build --release``` или ```cargo run --release```

---

## EN
### Snow Graphics System (hereinafter SGS) - a free open-source graphics engine written in `Rust` using `Vulkan`. ***COMMERCIAL USE IS PROHIBITED***
---
### Versions are designated by elements of the periodic table. You can view all releases [here](https://github.com/halva89/Snow-Graphics-System/releases)

1. **Alpha 1: Uranium**. The first triangle and shaders were created. Architecture not defined. [X] COMPLETED
2. **Alpha 2: Gold**. A basic parser for the `.sgss` file format was created. Architecture: Rusty Wolf [X] COMPLETED
3. **Alpha 3.0: Lithium**. A full-featured 2D shape parser (triangle, circle, square) is ready. The release includes a demo scene. Architecture: Red Wolf [X] COMPLETED
4. **Alpha 3.1: Lithium V2**. Animations are ready (alpha, bugs included). Full-featured `.sgss` parser, including `animation` and other shapes, as well as window settings. A camera with viewport calibration has been added. Exit with resource destruction [X] COMPLETED
5. **Pre-beta: Beryllium**. Bug fixes, basic drafts of 3D shapes. Multithreaded rendering and refactoring. [/] IN DEVELOPMENT
6. **Beta 1: Radium**. Shadow creation, full-featured camera. [|] IN PREPARATION
7. Versions will be expanded
---
### System requirements (latest version)
### Minimum:
- OS: Windows 10
- CPU: Core i3 / Ryzen 3
- RAM: 4 GB
- GPU: with Vulkan 1.0 support (older than 2015)

### Recommended:
- OS: Windows 10/11
- CPU: Core i5 / Ryzen 5
- RAM: 8 GB
- GPU: GeForce GTX / Radeon RX 5xx
---
### Installation
**Method 1 (binary):**
1. Download the latest, suitable, or desired version from [releases](https://github.com/halva89/Snow-Graphics-System/releases)
2. Extract the `release` folder
3. Open the folder and run `snow_graphics.exe`
   
**Method 2 (building from source):**
1. Install the `source` of the latest version and the Rust SDK
2. Navigate to the folder with the installed source code
3. Run `cargo build --release` or `cargo run --release`
