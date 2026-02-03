// Mermaid configuration for dark theme with improved visibility
mermaid.initialize({
  startOnLoad: true,
  theme: 'base',
  securityLevel: 'loose',
  themeVariables: {
    // Background and text
    primaryColor: '#3498db',
    primaryTextColor: '#ffffff',
    primaryBorderColor: '#5dade2',

    // Secondary colors
    secondaryColor: '#27ae60',
    secondaryTextColor: '#ffffff',
    secondaryBorderColor: '#58d68d',

    // Tertiary colors
    tertiaryColor: '#9b59b6',
    tertiaryTextColor: '#ffffff',
    tertiaryBorderColor: '#bb8fce',

    // Line and arrow colors - bright for dark backgrounds
    lineColor: '#adb5bd',

    // Background colors
    background: '#1e1e1e',
    mainBkg: '#2d2d2d',
    nodeBkg: '#3d3d3d',

    // Text
    textColor: '#e0e0e0',
    nodeTextColor: '#ffffff',

    // Flowchart specific
    clusterBkg: '#404040',
    clusterBorder: '#6c757d',

    // Notes
    noteBkgColor: '#3d3d3d',
    noteTextColor: '#e0e0e0',
    noteBorderColor: '#6c757d',

    // Actor lines in sequence diagrams
    actorLineColor: '#adb5bd',
    signalColor: '#adb5bd',

    // Git graph
    git0: '#3498db',
    git1: '#27ae60',
    git2: '#e74c3c',
    git3: '#f39c12',

    // Edge label background
    edgeLabelBackground: '#2d2d2d'
  },
  flowchart: {
    useMaxWidth: true,
    htmlLabels: true,
    curve: 'basis',
    diagramPadding: 8
  },
  sequence: {
    diagramMarginX: 50,
    diagramMarginY: 10,
    actorMargin: 50,
    width: 150,
    height: 65,
    boxMargin: 10,
    boxTextMargin: 5,
    noteMargin: 10,
    messageMargin: 35
  },
  gantt: {
    titleTopMargin: 25,
    barHeight: 20,
    barGap: 4,
    topPadding: 50
  }
});
