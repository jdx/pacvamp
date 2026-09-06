import { socialCard, writeSocialCard } from "./social-images.mjs";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitepress";

const configDir = dirname(fileURLToPath(import.meta.url));
const cargoToml = readFileSync(resolve(configDir, "../../Cargo.toml"), "utf8");
const versionMatch = cargoToml.match(
  /^\[workspace\.package\][\s\S]*?^\s*version\s*=\s*"([^"]+)"/m,
);
if (!versionMatch) {
  console.warn("Unable to find workspace package version in Cargo.toml");
}
const latestVersion = versionMatch?.[1] ?? "0.0.0";
const siteUrl = "https://pacvamp.com";
const description =
  "Preview package changes, review AUR recipes, and declare packages with pacvamp, a proof-of-concept pacman frontend.";

export default defineConfig({
  title: "pacvamp",
  description,
  lang: "en-US",
  head: [
    ["meta", { property: "og:type", content: "website" }],
    ["meta", { property: "og:site_name", content: "pacvamp" }],
    ["meta", { property: "og:locale", content: "en_US" }],
    ["meta", { property: "og:image:type", content: "image/png" }],
    ["meta", { property: "og:image:width", content: "1200" }],
    ["meta", { property: "og:image:height", content: "630" }],
    ["meta", { name: "twitter:card", content: "summary_large_image" }],
    ["meta", { name: "twitter:site", content: "@jdxcode" }],
    ["link", { rel: "icon", href: "/favicon.ico", sizes: "any" }],
    ["link", { rel: "icon", type: "image/png", sizes: "32x32", href: "/favicon-32x32.png" }],
    ["link", { rel: "icon", type: "image/svg+xml", href: "/favicon.svg" }],
    ["link", { rel: "apple-touch-icon", sizes: "180x180", href: "/apple-touch-icon.png" }],
    ["link", { rel: "manifest", href: "/site.webmanifest" }],
    ["meta", { name: "theme-color", content: "#17112b" }],
  ],
  transformHead: ({ pageData, title, description: pageDescription, siteConfig }) => {
    const heading = pageData.title || "pacvamp";
    const card = socialCard(heading);
    writeSocialCard(siteConfig.outDir, card);
    const image = new URL(card.path, `${siteUrl}/`).toString();
    const imageAlt = `${heading} — pacvamp docs`;
    const pagePath = pageData.relativePath
      .replace(/(^|\/)index\.md$/, "$1")
      .replace(/\.md$/, "");
    const url = new URL(pagePath, `${siteUrl}/`).toString();

    return [
      ["link", { rel: "canonical", href: url }],
      ["meta", { property: "og:url", content: url }],
      ["meta", { property: "og:image", content: image }],
      ["meta", { property: "og:image:alt", content: imageAlt }],
      ["meta", { name: "twitter:image", content: image }],
      ["meta", { name: "twitter:image:alt", content: imageAlt }],
      ["meta", { property: "og:title", content: title }],
      ["meta", { property: "og:description", content: pageDescription }],
      ["meta", { name: "twitter:title", content: title }],
      ["meta", { name: "twitter:description", content: pageDescription }],
      [
        "script",
        { type: "application/ld+json" },
        JSON.stringify({
          "@context": "https://schema.org",
          "@type": "WebPage",
          name: title,
          description: pageDescription,
          url,
          isPartOf: {
            "@type": "WebSite",
            name: "pacvamp",
            url: siteUrl,
          },
        }),
      ],
    ];
  },
  cleanUrls: true,
  lastUpdated: true,
  sitemap: {
    hostname: siteUrl,
  },
  themeConfig: {
    logo: { src: "/logo.svg", alt: "pacvamp" },
    nav: [
      { text: "Get started", link: "/getting-started" },
      { text: "Guides", link: "/packages" },
      { text: "Reference", items: [
        { text: "Client CLI", link: "/cli/pacvamp/" },
        { text: "Repository CLI", link: "/cli/pacvamp-repo/" },
        { text: "Configuration", link: "/configuration" },
        { text: "Specifications", link: "/spec/repository-feeds" },
      ] },
      { text: "Contribute", link: "/development" },
      { text: `v${latestVersion}`, link: "https://github.com/jdx/pacvamp/releases" },
    ],
    sidebar: [
      { text: "Start here", items: [
        { text: "Overview", link: "/" },
        { text: "Status and limitations", link: "/project-status" },
        { text: "Install pacvamp", link: "/install" },
        { text: "First steps", link: "/getting-started" },
      ] },
      { text: "Use pacvamp", items: [
        { text: "Package operations", link: "/packages" },
        { text: "Manifests", link: "/manifests" },
        { text: "Configuration", link: "/configuration" },
        { text: "Import a machine", link: "/migration" },
        { text: "AUR review and builds", link: "/aur" },
        { text: "Updates and blockers", link: "/update-policy" },
        { text: "Snapshots and rollback", link: "/snapshots" },
        { text: "Transaction recovery", link: "/recovery" },
      ] },
      { text: "Build and troubleshoot", collapsed: true, items: [
        { text: "Check active protections", link: "/protection-status" },
        { text: "Build controls", link: "/build-controls" },
        { text: "Clean-chroot builds", link: "/clean-chroot" },
        { text: "Build receipts and replay", link: "/build-receipts" },
        { text: "Cache retention", link: "/cache" },
      ] },
      { text: "Operate and integrate", collapsed: true, items: [
        { text: "Run a registry", link: "/operations/registry" },
        { text: "Trust roots", link: "/trust" },
        { text: "Omarchy adoption", link: "/adoption/omarchy" },
        { text: "OPR adoption", link: "/adoption/opr" },
        { text: "mise adoption", link: "/adoption/mise" },
      ] },
      { text: "Reference and contribute", collapsed: true, items: [
        { text: "Client CLI", link: "/cli/pacvamp/" },
        { text: "Repository CLI", link: "/cli/pacvamp-repo/" },
        { text: "Security model", link: "/security-model" },
        { text: "Architecture", link: "/architecture" },
        { text: "Design decisions", link: "/design-decisions" },
        { text: "Development", link: "/development" },
        { text: "Security acceptance tests", link: "/security-testing" },
        { text: "Roadmap", link: "https://github.com/jdx/pacvamp/blob/main/PLAN.md" },
      ] },
      { text: "Specifications", collapsed: true, items: [
        { text: "Packslip integration", link: "/spec/packslip" },
        { text: "Repository feeds", link: "/spec/repository-feeds" },
        { text: "Build provenance", link: "/spec/provenance" },
        { text: "Vendor pipeline", link: "/spec/vendor-pipeline" },
        { text: "AUR sync gate", link: "/spec/sync-gate" },
        { text: "Release train", link: "/spec/release-train" },
        { text: "Snapshot store", link: "/spec/snapshot-store" },
        { text: "Tool channel", link: "/spec/tool-channel" },
      ] },
    ],
    outline: "deep",
    search: {
      provider: "local",
    },
    socialLinks: [
      { icon: "github", link: "https://github.com/jdx/pacvamp" },
    ],
    editLink: {
      pattern: "https://github.com/jdx/pacvamp/edit/main/docs/:path",
    },
    footer: {
      message: "Released under the MIT License.",
    },
  },
});
