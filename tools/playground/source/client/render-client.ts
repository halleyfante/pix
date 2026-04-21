export interface RenderResult {
  image: Blob;
}

export interface RenderError {
  error: string;
}

export async function renderSource(source: string): Promise<Blob> {
  const response = await fetch("/render", {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ source }),
  });

  if (!response.ok) {
    const body: RenderError = await response.json();
    throw new Error(body.error);
  }

  return response.blob();
}
