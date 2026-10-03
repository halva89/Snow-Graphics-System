const os = require("os");
const path = require("path");
const CFG = process.env.XDG_CONFIG_HOME || path.join(os.homedir(), ".config");
const SKILL = path.join(CFG, "gigatool", "skills", "pptxgenjs-presentation");
const PptxGenJS = require(path.join(SKILL, "vendor", "pptxgenjs.bundle.cjs"));
const H = require(path.join(SKILL, "helpers", "index.cjs"));

const P = {
  bg: "131316",
  ink: "FFFFFF",
  muted: "9A9AA1",
  accent: "FF7A1A",
  line: "2A2A2E",
};
H.verifyPalette(P);
const FONT = "Inter";
const SERIF = "Georgia";
const COVER = "C:/Users/halva/Documents/GitHub/Snow-Graphics-System/mf-cover.jpg";

const pptx = new PptxGenJS();
pptx.layout = "LAYOUT_WIDE";
pptx.theme = { headFontFace: FONT, bodyFontFace: FONT };
const K = H.designKit(pptx, P, FONT);

const INK = P.ink, MUT = P.muted, ACC = P.accent, BG = P.bg, LINE = P.line;
const CARD = "1C1C20";

function finish(slide) {
  H.warnIfSlideTextOverflows(slide, pptx);
  H.warnIfSlideHasOverlaps(slide, pptx);
  H.warnIfSlideElementsOutOfBounds(slide, pptx);
}

function header(slide, eyebrow, title) {
  slide.addText(eyebrow.toUpperCase(), { x: 0.6, y: 0.4, w: 12, h: 0.3, fontFace: FONT, fontSize: 11, bold: true, color: ACC, charSpacing: 2 });
  slide.addText(title, { x: 0.6, y: 0.76, w: 12.1, h: 0.8, fontFace: FONT, fontSize: 26, bold: true, color: INK, valign: "top" });
}

function footer(slide, n) {
  slide.addText(String(n).padStart(2, "0"), { x: 12.7, y: 7.02, w: 0.6, h: 0.3, fontFace: SERIF, fontSize: 11, color: MUT, align: "right" });
  slide.addText("main.frame", { x: 0.6, y: 7.02, w: 3, h: 0.3, fontFace: FONT, fontSize: 9, color: MUT });
}

function card(slide, x, y, w, h, fill) {
  slide.addShape(pptx.ShapeType.rect, { x, y, w, h, fill: { color: fill || CARD }, line: { color: LINE, width: 1 }, rectRadius: 0.06 });
}

const OUT = "main-frame.pptx";

async function main() {
  // 1 — Cover
  const s1 = pptx.addSlide();
  s1.background = { color: BG };
  s1.addImage(H.imageSizingCrop(COVER, 8.4, 0, 4.93, 7.5));
  s1.addText("main.frame", { x: 0.7, y: 2.4, w: 7.2, h: 1.1, fontFace: FONT, fontSize: 48, bold: true, color: INK });
  s1.addText("графический движок на Rust и Vulkan", { x: 0.7, y: 3.55, w: 7.0, h: 0.6, fontFace: FONT, fontSize: 20, color: MUT });
  s1.addText("Один движок — любая платформа. Реальный рендер, физика, анимация и термальный режим.", { x: 0.7, y: 4.35, w: 7.0, h: 1.1, fontFace: FONT, fontSize: 15, color: MUT });
  s1.addText("v0.1 · Open source", { x: 0.7, y: 6.6, w: 6, h: 0.4, fontFace: FONT, fontSize: 12, color: ACC });
  finish(s1);

  // 2 — Mission (why)
  const s2 = pptx.addSlide();
  s2.background = { color: BG };
  header(s2, "Зачем", "Меньше зависимостей от чужих движков");
  card(s2, 0.6, 1.6, 12.13, 5.1, CARD);
  s2.addText("main.frame — собственный, полностью контролируемый графический стек.", { x: 1.0, y: 2.0, w: 11.3, h: 0.5, fontFace: FONT, fontSize: 18, color: INK, bold: true });
  for (const [i, t] of [
    ["Свой стек", "HTML и ядро пишем сами — не связаны лицензиями и чужим API."],
    ["Понятный формат", "Сцены описываются текстом .mff, читаются и правятся голыми руками."],
    ["Учебная ценность", "Vulkan изнутри: пайплайны, дескрипторы, шейдеры, свопчейны."],
    ["Основа для сервера", "Архитектуру готовим под будущий графический сервер и композер."],
  ].entries()) {
    const x = 1.0 + (i % 2) * 5.75;
    const y = 2.75 + Math.floor(i / 2) * 1.85;
    s2.addText(t[0], { x, y, w: 5.3, h: 0.4, fontFace: FONT, fontSize: 15, bold: true, color: ACC });
    s2.addText(t[1], { x, y: y + 0.4, w: 5.3, h: 1.15, fontFace: FONT, fontSize: 13, color: MUT, valign: "top" });
  }
  footer(s2, 2); finish(s2);

  // 3 — Why Rust + Vulkan
  const s3 = pptx.addSlide();
  s3.background = { color: BG };
  header(s3, "Стек", "Rust для надёжности · Vulkan для скорости");
  card(s3, 0.6, 1.6, 5.9, 5.1, CARD);
  s3.addText("Rust", { x: 1.0, y: 1.95, w: 5, h: 0.5, fontFace: FONT, fontSize: 20, bold: true, color: ACC });
  s3.addText([
    { text: "Безопасная память без сборщика мусора", options: { bullet: true, indent: 16 } },
    { text: "Скорость уровня C/C++ при контроле компилятора", options: { bullet: true, indent: 16 } },
    { text: "Многопоточный рендеринг на rayon", options: { bullet: true, indent: 16 } },
    { text: "Вся логика во владении — меньше неясных крашей", options: { bullet: true, indent: 16 } },
  ], { x: 1.0, y: 2.6, w: 5.2, h: 3.9, fontFace: FONT, fontSize: 13, color: INK, valign: "top", lineSpacingMultiple: 1.25 });
  card(s3, 6.85, 1.6, 5.88, 5.1, CARD);
  s3.addText("Vulkan", { x: 7.25, y: 1.95, w: 5, h: 0.5, fontFace: FONT, fontSize: 20, bold: true, color: ACC });
  s3.addText([
    { text: "Графика через ash — тонкий, полный контроль GPU", options: { bullet: true, indent: 16 } },
    { text: "Кроссплатформенно: Windows, Linux, macOS", options: { bullet: true, indent: 16 } },
    { text: "MSAA, FXAA, тени, термальный блюм", options: { bullet: true, indent: 16 } },
    { text: "Хотим — свой дисплей вместо окна ОС", options: { bullet: true, indent: 16 } },
  ], { x: 7.25, y: 2.6, w: 5.2, h: 3.9, fontFace: FONT, fontSize: 13, color: INK, valign: "top", lineSpacingMultiple: 1.25 });
  footer(s3, 3); finish(s3);

  // 4 — Architecture (2x2 cards)
  const s4 = pptx.addSlide();
  s4.background = { color: BG };
  header(s4, "Архитектура", "Модули, каждый за своё");
  const arch = [
    ["core · движок", "Engine, Scene, Object, Transform, камера и цикл кадра."],
    ["render · Vulkan", "Свопчейн, пайплайны, шейдеры, MSAA/FXAA, тени, отсечение."],
    ["physics · rapier3d", "Гравитация, упругие тела, статичные стены и кучи кубов."],
    ["scene_parser · .mff", "Текстовый формат: окно, объекты, анимации, физика, термаль."],
  ];
  for (const [i, a] of arch.entries()) {
    const x = 0.6 + (i % 2) * 6.12;
    const y = 1.6 + Math.floor(i / 2) * 2.55;
    card(s4, x, y, 5.9, 2.3, CARD);
    s4.addText(a[0], { x: x + 0.4, y: y + 0.3, w: 5.1, h: 0.4, fontFace: FONT, fontSize: 15, bold: true, color: ACC });
    s4.addText(a[1], { x: x + 0.4, y: y + 0.8, w: 5.1, h: 1.2, fontFace: FONT, fontSize: 13, color: MUT, valign: "top" });
  }
  footer(s4, 4); finish(s4);

  // 5 — .mff format (code block + notes)
  const s5 = pptx.addSlide();
  s5.background = { color: BG };
  header(s5, "Формат сцен", "Сцена — это текстовый файл .mff");
  const code = [
    'window 1600 900',
    'title "main.frame"',
    'camera true',
    'thermal true',
    '',
    'temp 3500',
    'position 0 0.5 0',
    'cube',
    '1.0',
    '',
    'temp 15',
    'position -2 0.5 0',
    'sphere',
    '0.6',
    '16',
  ];
  H.addCodeBlock(s5, pptx, code.join("\n"), { lang: "text", x: 0.6, y: 1.6, w: 7.4, theme: "dark" });
  s5.addText("Никакой бинарной схемы:", { x: 8.4, y: 1.7, w: 4.3, h: 0.4, fontFace: FONT, fontSize: 14, bold: true, color: INK });
  s5.addText([
    { text: "Объекты, анимации, физика и окно — текстом", options: { bullet: true, indent: 16 } },
    { text: "temp <число> — температура объекта", options: { bullet: true, indent: 16 } },
    { text: "Файл правится и перезагружается на лету", options: { bullet: true, indent: 16 } },
  ], { x: 8.4, y: 2.2, w: 4.3, h: 3.2, fontFace: FONT, fontSize: 13, color: MUT, valign: "top", lineSpacingMultiple: 1.25 });
  footer(s5, 5); finish(s5);

  // 6 — Rendering (big-stat + bullets)
  const s6 = pptx.addSlide();
  s6.background = { color: BG };
  header(s6, "Рендеринг", "Сцена собирается в один пул геометрии");
  card(s6, 0.6, 1.6, 6.0, 5.1, CARD);
  s6.addText("одна\nгеометрия →\nна кадр", { x: 1.0, y: 2.4, w: 5.2, h: 2.4, fontFace: SERIF, fontSize: 34, bold: true, color: ACC });
  s6.addText("Vertex/index-буфер собирается один раз, объекты рисуются одним-двумя вызовами.", { x: 1.0, y: 5.2, w: 5.3, h: 1.1, fontFace: FONT, fontSize: 13, color: MUT, valign: "top" });
  s6.addText([
    { text: "Динамические viewport и scissor под любой размер окна", options: { bullet: true, indent: 16 } },
    { text: "Карта теней и мультисэмплинг MSAA", options: { bullet: true, indent: 16 } },
    { text: "Пост-обработка FXAA — сглаживание под любой GPU", options: { bullet: true, indent: 16 } },
  ], { x: 7.0, y: 2.0, w: 5.6, h: 3.0, fontFace: FONT, fontSize: 14, color: INK, valign: "top", lineSpacingMultiple: 1.25 });
  footer(s6, 6); finish(s6);

  // 7 — Culling
  const s7 = pptx.addSlide();
  s7.background = { color: BG };
  header(s7, "Оптимизация", "Не рисуем то, чего не видно");
  card(s7, 0.6, 1.6, 6.0, 5.1, CARD);
  s7.addText("View-frustum culling", { x: 1.0, y: 2.0, w: 5.2, h: 0.5, fontFace: FONT, fontSize: 18, bold: true, color: ACC });
  s7.addText("У каждого объекта есть AABB. Шесть плоскостей фрустума камеры выбирают только видимые — невидимое не попадает ни в буфер, ни в draw-вызовы.", { x: 1.0, y: 2.6, w: 5.3, h: 2.0, fontFace: FONT, fontSize: 14, color: MUT, valign: "top" });
  s7.addText("Один пул отрисовки", { x: 1.0, y: 4.8, w: 5.2, h: 0.5, fontFace: FONT, fontSize: 18, bold: true, color: ACC });
  s7.addText("Всё видимое складывается в общий буфер и рисуется минимальным числом переключений.", { x: 1.0, y: 5.3, w: 5.3, h: 1.2, fontFace: FONT, fontSize: 14, color: MUT, valign: "top" });
  s7.addShape(pptx.ShapeType.ellipse, { x: 8.0, y: 2.6, w: 3.6, h: 3.6, fill: { color: "1A1A1F" }, line: { color: ACC, width: 2 } });
  s7.addText("fr", { x: 8.0, y: 3.1, w: 3.6, h: 1.3, fontFace: SERIF, fontSize: 58, bold: true, color: ACC, align: "center" });
  s7.addText("объекты вне камеры\nотсекаются до сборки", { x: 8.0, y: 4.9, w: 3.6, h: 1.1, fontFace: FONT, fontSize: 12, color: MUT, align: "center" });
  footer(s7, 7); finish(s7);

  // 8 — Physics
  const s8 = pptx.addSlide();
  s8.background = { color: BG };
  header(s8, "Физика", "Мы строим миры, которые ведут себя честно");
  const rx = ["Гравитация и силы", "Упругость соударений (restitution)", "Инерция и масса тел", "Статичные стены и кучи кубов"];
  for (const [i, t] of rx.entries()) {
    const x = 0.6 + (i % 2) * 6.12;
    const y = 1.6 + Math.floor(i / 2) * 2.55;
    card(s8, x, y, 5.9, 2.3, CARD);
    s8.addShape(pptx.ShapeType.ellipse, { x: x + 0.4, y: y + 0.75, w: 0.9, h: 0.9, fill: { color: ACC } });
    s8.addText(String(i + 1), { x: x + 0.4, y: y + 0.9, w: 0.9, h: 0.6, fontFace: SERIF, fontSize: 26, bold: true, color: BG, align: "center" });
    s8.addText(t, { x: x + 1.5, y: y + 0.85, w: 4.1, h: 0.9, fontFace: FONT, fontSize: 15, bold: true, color: INK, valign: "top" });
  }
  footer(s8, 8); finish(s8);

  // 9 — Animation
  const s9 = pptx.addSlide();
  s9.background = { color: BG };
  header(s9, "Анимация", "Клавиши, интерполяция, движение");
  card(s9, 0.6, 1.6, 6.0, 5.1, CARD);
  s9.addText("Keyframe-система", { x: 1.0, y: 2.0, w: 5.2, h: 0.5, fontFace: FONT, fontSize: 18, bold: true, color: ACC });
  for (const [i, t] of ["Время и позиция в каждой ключевой точке", "Цвет тоже анимируется — объект «дышит»", "Плавная интерполяция между кадрами", "Параллельная обработка объектов через rayon"].entries()) {
    s9.addText(t, { x: 1.0, y: 2.7 + i * 1.0, w: 5.2, h: 0.9, fontFace: FONT, fontSize: 14, color: INK, valign: "top", bullet: true });
  }
  card(s9, 6.85, 1.6, 5.88, 5.1, CARD);
  s9.addText("example .mff", { x: 7.25, y: 1.95, w: 5, h: 0.4, fontFace: FONT, fontSize: 13, bold: true, color: MUT });
  H.addCodeBlock(s9, pptx, [
    'animate cube',
    '  keyframe 0.0',
    '  pos 0 3 0',
    '  color 1 0 0',
    '  keyframe 2.0',
    '  pos 3 5 0',
    '  color 1 1 0',
    '}',
  ].join("\n"), { lang: "text", x: 7.1, y: 2.5, w: 5.4, theme: "dark" });
  footer(s9, 9); finish(s9);

  // 10 — Thermal highlight
  const s10 = pptx.addSlide();
  s10.background = { color: BG };
  header(s10, "Тепловизор", "Смотри на мир в тепле");
  s10.addText("Холод = тёмно-синий · тёплый = красный · горячий = жёлтый · очень горячий = белый", { x: 0.6, y: 1.62, w: 12.1, h: 0.5, fontFace: FONT, fontSize: 15, color: MUT });
  const grad = ["001133", "003366", "CC0000", "FF6600", "FFCC00", "FFFFFF"];
  const gw = 12.13 / grad.length;
  for (const [i, c] of grad.entries()) {
    s10.addShape(pptx.ShapeType.rect, { x: 0.6 + i * gw, y: 2.2, w: gw, h: 2.6, fill: { color: c }, line: { color: LINE, width: 1 } });
  }
  s10.addText("Температура объекта задаётся прямо в .mff:", { x: 0.6, y: 5.1, w: 12, h: 0.4, fontFace: FONT, fontSize: 14, color: INK, bold: true });
  s10.addText("temp <число> — перед фигурой. Включение режима — thermal true.", { x: 0.6, y: 5.6, w: 12, h: 0.5, fontFace: FONT, fontSize: 14, color: MUT });
  footer(s10, 10); finish(s10);

  // 11 — Thermal mechanics
  const s11 = pptx.addSlide();
  s11.background = { color: BG };
  header(s11, "Тепловизор · механика", "Сияние и распределение тепла");
  card(s11, 0.6, 1.6, 5.9, 5.1, CARD);
  s11.addText("Настоящее сияние", { x: 1.0, y: 1.95, w: 5.1, h: 0.5, fontFace: FONT, fontSize: 17, bold: true, color: ACC });
  s11.addText([
    { text: "Аддитивный блюм вокруг горячих объектов", options: { bullet: true, indent: 16 } },
    { text: "Два кольца: тёплый ореол + белое ядро", options: { bullet: true, indent: 16 } },
    { text: "Почти чёрный фон для контраста", options: { bullet: true, indent: 16 } },
    { text: "Объекты светят сами — без диффуза и теней", options: { bullet: true, indent: 16 } },
  ], { x: 1.0, y: 2.6, w: 5.2, h: 3.8, fontFace: FONT, fontSize: 13, color: INK, valign: "top", lineSpacingMultiple: 1.25 });
  card(s11, 6.85, 1.6, 5.88, 5.1, CARD);
  s11.addText("Распределение тепла", { x: 7.25, y: 1.95, w: 5.1, h: 0.5, fontFace: FONT, fontSize: 17, bold: true, color: ACC });
  s11.addText([
    { text: "Каждый объект имеет свой радиус влияния", options: { bullet: true, indent: 16 } },
    { text: "Динамическая диффузия во времени (dt)", options: { bullet: true, indent: 16 } },
    { text: "Горячий объект греет соседей — тепло идёт по цепочке", options: { bullet: true, indent: 16 } },
    { text: "Шум датчика на фрагменте — «приборный» вид", options: { bullet: true, indent: 16 } },
  ], { x: 7.25, y: 2.6, w: 5.2, h: 3.8, fontFace: FONT, fontSize: 13, color: INK, valign: "top", lineSpacingMultiple: 1.25 });
  footer(s11, 11); finish(s11);

  // 12 — cross-platform
  const s12 = pptx.addSlide();
  s12.background = { color: BG };
  header(s12, "Кроссплатформенность", "Один код — Windows, Linux, macOS");
  const plats = [
    ["Windows", "основная сборочная платформа разработки"],
    ["Linux · x86_64 + ARM", "apt/dnf, GLFW X11+Wayland, Vulkan"],
    ["macOS · Apple Silicon", "Vulkan через Metal — MoltenVK"],
  ];
  for (const [i, p] of plats.entries()) {
    const y = 1.6 + i * 1.7;
    card(s12, 0.6, y, 12.13, 1.45, CARD);
    s12.addText(p[0], { x: 1.0, y: y + 0.28, w: 4.2, h: 0.9, fontFace: FONT, fontSize: 17, bold: true, color: ACC, valign: "middle" });
    s12.addText(p[1], { x: 5.4, y: y + 0.28, w: 6.9, h: 0.9, fontFace: FONT, fontSize: 14, color: MUT, valign: "middle" });
  }
  s12.addText("CI-сборка · локальные скрипты build-linux.sh / build-macos.sh", { x: 0.6, y: 6.72, w: 11, h: 0.3, fontFace: FONT, fontSize: 12, color: MUT });
  footer(s12, 12); finish(s12);

  // 13 — Roadmap (timeline)
  const s13 = pptx.addSlide();
  s13.background = { color: BG };
  header(s13, "Дорожная карта", "От движка к платформе");
  const steps = [
    ["Beta", "стабильный релиз, кросплатформенность, заморозка .mff"],
    ["Редактор сцен", "визуальный редактор для формата .mff"],
    ["Графический сервер", "движок как композер/дисплей-сервер на своей ОС"],
    ["API и моды", "модульность и расширения для сообщества"],
  ];
  s13.addShape(pptx.ShapeType.line, { x: 0.9, y: 3.0, w: 11.5, h: 0, line: { color: LINE, width: 3 } });
  for (const [i, st] of steps.entries()) {
    const x = 0.7 + i * 3.05;
    s13.addShape(pptx.ShapeType.ellipse, { x: x + 0.9, y: 2.78, w: 0.45, h: 0.45, fill: { color: ACC } });
    s13.addText(st[0], { x, y: 1.75, w: 2.8, h: 0.5, fontFace: FONT, fontSize: 14, bold: true, color: INK, align: "center" });
    s13.addText(st[1], { x, y: 3.6, w: 2.8, h: 1.9, fontFace: FONT, fontSize: 13, color: MUT, align: "center", valign: "top" });
  }
  footer(s13, 13); finish(s13);

  // 14 — Stats / big statement
  const s14 = pptx.addSlide();
  s14.background = { color: BG };
  header(s14, "Разработано внутри проекта", "Что уже умеет main.frame");
  s14.addShape(pptx.ShapeType.rect, { x: 0.6, y: 1.7, w: 12.13, h: 3.6, fill: { color: "1A1A1F" }, line: { color: ACC, width: 1.5 }, rectRadius: 0.08 });
  s14.addText("Vulkan · тени · MSAA · FXAA", { x: 0.6, y: 2.2, w: 12.13, h: 0.6, fontFace: FONT, fontSize: 16, color: INK, align: "center" });
  s14.addText("culling · физика rapier · анимация · формат .mff", { x: 0.6, y: 2.9, w: 12.13, h: 0.6, fontFace: FONT, fontSize: 16, color: MUT, align: "center" });
  s14.addText("термальный режим · сияние · распределение тепла · шум датчика", { x: 0.6, y: 3.6, w: 12.13, h: 0.6, fontFace: FONT, fontSize: 16, color: ACC, align: "center" });
  s14.addText("И это только начало.", { x: 0.6, y: 5.6, w: 12.13, h: 0.6, fontFace: FONT, fontSize: 18, bold: true, color: INK, align: "center" });
  footer(s14, 14); finish(s14);

  // 15 — Closing
  const s15 = pptx.addSlide();
  s15.background = { color: "1A1A1F" };
  s15.addText("main.frame", { x: 0.6, y: 2.6, w: 12, h: 1.0, fontFace: FONT, fontSize: 40, bold: true, color: INK, align: "center" });
  s15.addText("Open source · Rust · Vulkan · .mff · термальный режим", { x: 0.6, y: 3.8, w: 12, h: 0.6, fontFace: FONT, fontSize: 16, color: ACC, align: "center" });
  s15.addText("Попробуйте сцену assets/thermal.mff — и посмотрите мир в тепле.", { x: 0.6, y: 4.6, w: 12, h: 0.6, fontFace: FONT, fontSize: 15, color: MUT, align: "center" });
  s15.addText("Спасибо", { x: 0.6, y: 5.6, w: 12, h: 0.7, fontFace: SERIF, fontSize: 30, bold: true, color: INK, align: "center" });
  finish(s15);

  await pptx.writeFile({ fileName: OUT });
  await H.recompressPptx(OUT);
  console.log("written: " + OUT);
}
main().catch((e) => { console.error(e); process.exit(1); });