// Mermaid configuration with LIGHT background for visibility
mermaid.initialize({
  startOnLoad: true,
  theme: 'base',
  securityLevel: 'loose',
  themeVariables: {
    // LIGHT background for visibility on dark pages
    background: '#ffffff',
    mainBkg: '#f8f9fa',

    // Primary colors - blue
    primaryColor: '#4a90d9',
    primaryTextColor: '#1a1a1a',
    primaryBorderColor: '#2171b5',

    // Secondary colors - green
    secondaryColor: '#6cc644',
    secondaryTextColor: '#1a1a1a',
    secondaryBorderColor: '#4a9c2d',

    // Tertiary colors - purple
    tertiaryColor: '#a855f7',
    tertiaryTextColor: '#1a1a1a',
    tertiaryBorderColor: '#7c3aed',

    // Lines - DARK for visibility on light background
    lineColor: '#333333',

    // Node backgrounds - light
    nodeBkg: '#e8f4fd',
    nodeTextColor: '#1a1a1a',
    nodeBorder: '#2171b5',

    // Text - dark for light background
    textColor: '#1a1a1a',

    // Clusters
    clusterBkg: '#f0f4f8',
    clusterBorder: '#5a6c7d',

    // Notes
    noteBkgColor: '#fff8dc',
    noteTextColor: '#1a1a1a',
    noteBorderColor: '#d4a574',

    // Sequence diagram
    actorBkg: '#e8f4fd',
    actorBorder: '#2171b5',
    actorTextColor: '#1a1a1a',
    actorLineColor: '#333333',
    signalColor: '#333333',
    signalTextColor: '#1a1a1a',
    labelTextColor: '#1a1a1a',

    // Edge labels
    edgeLabelBackground: '#ffffff',

    // Gantt
    sectionBkgColor: '#e8f4fd',
    altSectionBkgColor: '#f0f4f8',
    gridColor: '#cbd5e1',
    todayLineColor: '#dc2626',
    critBkgColor: '#fee2e2',
    critBorderColor: '#dc2626',
    doneBkgColor: '#d1fae5',
    doneBorderColor: '#059669',

    // Git graph
    git0: '#4a90d9',
    git1: '#6cc644',
    git2: '#dc2626',
    git3: '#f59e0b'
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
