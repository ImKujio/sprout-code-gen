export default {
    outPut : '{{value}}\n' +
    '{{data.key}}\n' +
    '{{data[\'key\']}}\n' +
    '{{a ? b : c}}\n' +
    '{{a || b}}\n' +
    '{{a + b}}',
    outputOrg : '{{@ value }}',
    condition : '{{if value}} ... {{/if}}\n' +
        '{{if v1}} ... {{else if v2}} ... {{/if}}',
    circulation : '{{each target}}\n' +
        '    {{$index}} {{$value}}\n' +
        '{{/each}}',
    variable : '{{set temp = data.sub.content}}',
    nameCase : '大驼峰: {{name | CamelCase}}\n' +
        '小驼峰：{{name | camelCase}}\n' +
        '蛇形：{{name | snake_case}}',
    inline : '{{line}}{{/line}}'
}