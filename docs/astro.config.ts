import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import { ion } from "starlight-ion-theme";

export default defineConfig({
    site: 'https://dpscript.openvoxel.studio',
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
                    label: "Guides",
                    items: [
                        {
                            slug: "guides/getting_started",
                        },
                    ],
                },
                {
                    label: "Documentation",
                    items: [
                        {
                            slug: "docs/file_structure",
                        },
                    ],
                },
            ],
            plugins: [ion()],
        }),
    ],
});
