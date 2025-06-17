import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import { ion } from "starlight-ion-theme";
import * as fs from "fs";
import * as path from "path";
import { fileURLToPath } from "url";

export default defineConfig({
    site: "https://dpscript.openvoxel.studio",
    integrations: [
        starlight({
            title: "DPScript Docs",
            social: [
                {
                    icon: "github",
                    label: "GitHub",
                    href: "https://github.com/OpenVoxelStudios/DPScript",
                },
                {
                    icon: "discord",
                    label: "Discord",
                    href: "https://discord.gg/Xhvb2wujVh",
                },
            ],
            sidebar: [
                {
                    label: "[tabler:home] Home",
                    slug: "index",
                },
                {
                    label: "[tabler:globe] Resources",
                    slug: "resources",
                },
                {
                    label: "[tabler:road] Roadmap",
                    slug: "roadmap",
                },
                {
                    label: "[tabler:book] Guides",
                    autogenerate: {
                        directory: "guides",
                    },
                },
                {
                    label: "[tabler:code] Documentation",
                    autogenerate: {
                        directory: "docs",
                    },
                },
                {
                    label: "[tabler:sparkles] Design",
                    autogenerate: {
                        directory: "design",
                    },
                },
            ],
            expressiveCode: {
                shiki: {
                    langs: [
                        JSON.parse(
                            fs.readFileSync(
                                path.join(
                                    path.dirname(
                                        fileURLToPath(import.meta.url),
                                    ),
                                    "../vscode/syntaxes/dpscript.tmLanguage.json",
                                ),
                                "utf-8",
                            ),
                        ),
                    ],

                    langAlias: {
                        dps: "DPScript",
                        dpscript: "DPScript",
                    },
                },
            },
            plugins: [
                ion({
                    icons: {
                        include: { tabler: ["*"] },
                    },
                }) as any,
            ],
            customCss: ["@fontsource/jetbrains-mono", "/src/styles/theme.css"],
        }),
    ],
});
