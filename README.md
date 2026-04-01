# Ratatui

## Введение в ratatui

```shell
cargo r --bin intro
```

## Layout ratatui

```shell
cargo r --bin layouts -- -l positional
cargo r --bin layouts -- -l constraint
cargo r --bin layouts -- -l flex
cargo r --bin layouts -- -l nested
```

## Widgets ratatui

```shell
cargo r --bin widgets -- -w text
cargo r --bin widgets -- -w list
cargo r --bin widgets -- -w data
```


## Style ratatui

```shell
cargo r --bin styles -- -s text
cargo r --bin styles -- -s borders
cargo r --bin styles -- -s colors
```

## Custom widgets

```shell
cargo r --bin custom-widgets -- -w explorer
cargo r --bin custom-widgets -- -w qr-code
cargo r --bin custom-widgets -- -w pie-chart
cargo r --bin custom-widgets -- -w custom-widget
cargo r --bin custom-widgets -- -w statefull-widget
```

## Animations

```shell
cargo r --bin animations -- -a minimal
cargo r --bin animations -- -a paint
cargo r --bin animations -- -a combined
```

https://junkdog.github.io/tachyonfx-ftl/

## Architecture

```shell
cargo r --bin arch -- -a elm
cargo r --bin arch -- -a component
cargo r --bin arch -- -a flux
```
