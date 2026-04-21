import { configuration } from "@halleyfante/documentation";

export default {
  ...configuration({
    name: "Pix",
    site: "https://pix.halleyfante.com",
    sections: [
      { slug: "", label: "Overview" },
      { slug: "getting-started", label: "Getting Started" },
      { slug: "grid", label: "Grid" },
      { slug: "colors", label: "Colors" },
      { slug: "expressions", label: "Expressions" },
      { slug: "drawing", label: "Drawing" },
      { slug: "shapes", label: "Shapes" },
      { slug: "exporting", label: "Exporting" },
      { slug: "extension", label: "Extension" },
      { slug: "playground", label: "Playground" },
      { slug: "examples", label: "Examples" },
    ],
  }),
  outDir: "./distribution",
};
