# Third-party notices

Darkfactory's original code and documentation are MIT licensed. Bundled and
adapted material retains its original attribution and applicable notices.

| Material | Upstream | Notice |
| --- | --- | --- |
| TDD and related design/interview skill material | [Matt Pocock's skills](https://github.com/mattpocock/skills) | [MIT, Matt Pocock](LICENSES/mattpocock--skills.txt) |
| React performance guide in `skills/react-perf` | [Vercel React Best Practices](https://github.com/vercel-labs/agent-skills/tree/main/skills/react-best-practices) | [Upstream MIT declaration and attribution](LICENSES/vercel-react-best-practices-notice.md) |
| Web Interface Guidelines appended to `skills/react-perf` | [Vercel Labs](https://github.com/vercel-labs/web-interface-guidelines) | [MIT, Vercel Labs](LICENSES/vercel-labs--web-interface-guidelines.txt) |
| UI-system skill | [shadcn/ui](https://github.com/shadcn-ui/ui) | [MIT, shadcn](LICENSES/shadcn-ui--ui.txt) |
| Dispatcher supervision design/reference implementation | [buildoak/agent-mux](https://github.com/buildoak/agent-mux) | [MIT, buildoak](LICENSES/buildoak--agent-mux.txt) |
| OpenSpec integration and historical generated commands | [OpenSpec](https://github.com/Fission-AI/OpenSpec) | [MIT, OpenSpec Contributors](LICENSES/Fission-AI--OpenSpec.txt) |

Notices were checked against upstream sources on 2026-11-10. This date identifies
the notice review, not a claim that the bundled skills match the latest upstream
code. The skills have local adaptations.

The mission workflow also draws on Factory's mission architecture and published
software-design ideas. Those influences are explained in [provenance](docs/provenance.md);
Darkfactory is not affiliated with or endorsed by the referenced projects.

Cargo dependencies retain their own licenses. They are resolved by the committed
`Cargo.lock`; adding MIT to this repository does not relicense those dependencies.
The crate has `publish = false`: this release prepares a source repository, not
a crates.io package.
