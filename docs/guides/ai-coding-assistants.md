# AI Coding Assistants

The Kreuzcrawl plugin lives in the [`kreuzberg-dev/plugins`](https://github.com/kreuzberg-dev/plugins) marketplace. It ships the Kreuzcrawl agent skills (site crawling, HTML→Markdown scraping, headless-Chrome fallback) plus the `kreuzcrawl` MCP server, and works with every major coding agent.

The plugin shells out to the `kreuzcrawl` CLI. Install it from the [Installation](../getting-started/installation.md) guide (for example, `brew install kreuzberg-dev/tap/kreuzcrawl`) before driving the crawler from an assistant.

## Installing

Pick your harness below.

<details open>
<summary><strong>Claude Code</strong></summary>

```text
/plugin marketplace add kreuzberg-dev/plugins
/plugin install kreuzcrawl@kreuzberg
```

</details>

<details>
<summary><strong>Codex CLI</strong></summary>

```text
/plugins add https://github.com/kreuzberg-dev/plugins
```

Then search for `kreuzcrawl` and select **Install Plugin**.
</details>

<details>
<summary><strong>Cursor</strong></summary>

Settings → Plugins → Add from URL → `https://github.com/kreuzberg-dev/plugins`, then select **kreuzcrawl**.
</details>

<details>
<summary><strong>Gemini CLI</strong></summary>

```text
gemini extensions install https://github.com/kreuzberg-dev/plugins
```

</details>

<details>
<summary><strong>Factory Droid</strong></summary>

```text
droid plugin marketplace add https://github.com/kreuzberg-dev/plugins
droid plugin install kreuzcrawl@kreuzberg
```

</details>

<details>
<summary><strong>GitHub Copilot CLI</strong></summary>

```text
copilot plugin marketplace add https://github.com/kreuzberg-dev/plugins
copilot plugin install kreuzcrawl@kreuzberg
```

</details>

<details>
<summary><strong>opencode</strong></summary>

Add the package to `opencode.json`:

```json
{
  "$schema": "https://opencode.ai/config.json",
  "plugin": ["@kreuzberg/opencode-kreuzcrawl"]
}
```

</details>
