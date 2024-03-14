"use client";

import { ThemeProvider as NextThemeProvider } from "next-themes";
import { ComponentProps } from "react";

export const ThemeProvider = ({
  ...props
}: ComponentProps<typeof NextThemeProvider>) => (
  <NextThemeProvider {...props} />
);
