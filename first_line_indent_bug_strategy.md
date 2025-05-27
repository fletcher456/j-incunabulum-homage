# First Line Indent Bug Strategy

## Problem Statement
Matrix formatting shows an unwanted indent on the first line only. The leading newline hack works but is not an elegant solution.

## Evidence
- **With debug `&` characters**: No first-line indent visible (`&0&&1&&2&&3`)
- **With spaces**: First line shows extra indent (`  0  1  2  3`)
- **With leading newline**: Problem eliminated (matrix starts on second line)

## Root Cause Analysis
The issue is HTML/CSS related, not Rust formatting. The matrix calculations and alignment logic work perfectly.

## Suspected Causes (Priority Order)

### 1. Browser Text-Indent Inheritance
**Issue**: Browser may apply automatic text-indent to first line of content
**Investigation**: Check if parent elements have text-indent CSS rules
**Solution**: Add `text-indent: 0 !important` to override inheritance

### 2. CSS Box Model Padding/Margin
**Issue**: Message container padding affects first line differently
**Investigation**: Examine `.message` class padding and margin rules
**Solution**: Override padding specifically for matrix output

### 3. Paragraph Auto-Formatting
**Issue**: Browser treating first line as paragraph start
**Investigation**: Check if content is wrapped in `<p>` tags
**Solution**: Use `<div>` or `<pre>` wrapper instead

### 4. Font Rendering Quirks
**Issue**: Monospace font rendering differences on first line
**Investigation**: Test with different font families
**Solution**: Force consistent font rendering with CSS

## Investigation Steps

### Step 1: CSS Inheritance Analysis
```css
/* Add to .output class */
.output * {
    text-indent: 0 !important;
    margin-left: 0 !important;
}
```

### Step 2: Container Override
```css
/* Override message container for matrix output */
.output {
    padding: 0;
    margin: 0;
    display: block;
}
```

### Step 3: HTML Structure Check
- Verify output isn't wrapped in `<p>` tags
- Ensure proper `<pre>` or `<div>` container usage
- Check for inherited styles from parent elements

### Step 4: Browser DevTools Investigation
- Inspect computed styles on first line vs subsequent lines
- Check for automatic browser styling
- Identify specific CSS rule causing indent

## Potential Solutions (Best to Worst)

### A. CSS Reset (Preferred)
```css
.output {
    all: unset;
    display: block;
    color: #006600;
    white-space: pre;
    font-family: monospace;
}
```

### B. Negative Margin Compensation
```css
.output {
    margin-left: -8px; /* Compensate for container padding */
}
```

### C. HTML Structure Change
- Wrap matrix output in `<pre>` tags instead of `<div>`
- Use CSS `display: contents` to bypass container styling

### D. JavaScript Post-Processing
- Detect matrix output and apply special CSS class
- Programmatically remove leading spaces

## Current Workaround
- Leading newline forces matrix to start on line 2
- Avoids first-line formatting issues
- Should be replaced with proper CSS fix

## Success Criteria
- Matrix displays with perfect left alignment
- No leading newline needed
- All rows start at same horizontal position
- Solution works across browsers

## Risk Assessment
- **Low Risk**: CSS-only changes
- **Medium Risk**: HTML structure modifications
- **High Risk**: JavaScript-based solutions

The issue is definitively in the HTML/CSS rendering layer, not the Rust matrix formatting logic.