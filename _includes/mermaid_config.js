// Mermaid configuration with soft light background
mermaid.initialize({
  startOnLoad: true,
  theme: 'base',
  securityLevel: 'loose',
  themeVariables: {
    // Light background (slightly darker for better contrast)
    background: '#e2e6eb',
    mainBkg: '#dce0e5',

    // Primary colors - muted blue
    primaryColor: '#5a9bd5',
    primaryTextColor: '#2c3e50',
    primaryBorderColor: '#4a89c0',

    // Secondary colors - muted green
    secondaryColor: '#7ec88b',
    secondaryTextColor: '#2c3e50',
    secondaryBorderColor: '#5fb06f',

    // Tertiary colors - muted purple
    tertiaryColor: '#b08ed0',
    tertiaryTextColor: '#2c3e50',
    tertiaryBorderColor: '#9575b5',

    // Lines - dark gray for good contrast
    lineColor: '#4a5568',

    // Node backgrounds - soft tones
    nodeBkg: '#cdd5e0',
    nodeTextColor: '#2c3e50',
    nodeBorder: '#4a89c0',

    // Text - dark gray (not pure black)
    textColor: '#2c3e50',

    // Clusters
    clusterBkg: '#d5dae0',
    clusterBorder: '#7a8a9a',

    // Notes - soft cream
    noteBkgColor: '#ebe5d8',
    noteTextColor: '#2c3e50',
    noteBorderColor: '#c9b896',

    // Sequence diagram
    actorBkg: '#cdd5e0',
    actorBorder: '#4a89c0',
    actorTextColor: '#2c3e50',
    actorLineColor: '#4a5568',
    signalColor: '#4a5568',
    signalTextColor: '#2c3e50',
    labelTextColor: '#2c3e50',

    // Edge labels
    edgeLabelBackground: '#e2e6eb',

    // Gantt
    sectionBkgColor: '#dde4ed',
    altSectionBkgColor: '#e8ebef',
    gridColor: '#b8c4d0',
    todayLineColor: '#e05252',
    critBkgColor: '#f8d7da',
    critBorderColor: '#dc3545',
    doneBkgColor: '#d4edda',
    doneBorderColor: '#28a745',

    // Git graph
    git0: '#5a9bd5',
    git1: '#7ec88b',
    git2: '#e05252',
    git3: '#f0ad4e'
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
