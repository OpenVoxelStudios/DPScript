import { defineConfig } from "astro/config";
import starlight from "@astrojs/starlight";
import { ion } from "starlight-ion-theme";

export default defineConfig({
    integrations: [
        starlight({
            title: "DPScript Docs",
            social: [
                {
                    icon: "github",
                    label: "GitHub",
                    href: "https://github.com/OpenVoxelStudios/DPScript",
                },
            ],
            sidebar: [
                {
                    label: "Docs",
                    items: [
                        {
                            label: "Getting Started",
                            slug: "guides/getting_started",
                        },
                    ],
                },
                {
                    label: "Reference",
                    autogenerate: { directory: "reference" },
                },
            ],
            plugins: [ion()],
        }),
    ],
});
