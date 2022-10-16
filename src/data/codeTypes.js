export default [
  {
    name: "If", value: "if", description: "Executes the code inside if the condition is true.", parameters: [
      {
        name: "Condition", value: "condition", description: "The condition to check.", type: "condition", defaultValue: {
          type: "boolean",
          value: true,
        },
      },
    ],
    codeInside: [
      { name: "Then", value: "then", description: "The code to execute if the condition is true.", },
      { name: "Else", value: "else", description: "The code to execute if the condition is false.", },
    ], contentText: (parameters, parsers) => {
      return [`If`, ...parsers.parseCondition(parameters["condition"])];
    },
  },
  {
    name: "Function call", value: "function", description: "Executes the code inside the function it calls.", parameters: [
      { name: "Function", value: "function", type: "function", description: "The function to call.", },
    ], contentText: (parameters, parsers) => {
      return [`Call the function`, {type: "string", string: parameters["function"]}];
    },
  },
  {
    name: "Repeat from to loop", value: "fromtoloop", description: "Executes the code inside the loop for each number between 2 numbers", parameters: [
      { name: "Start", value: "start", description: "The starting number of the loop", type: "number", defaultValue: 0, },
      { name: "End", value: "end", description: "The ending number of the loop", type: "number", defaultValue: 10, },
      { name: "Step", value: "step", description: "How much the loop changes by each iteration", type: "number", defaultValue: 1, },
    ],
    codeInside: [
      { name: "Loop iteration", value: "loop", description: "The code to execute for each iteration.", },
    ],
    variables: [
      { name: "Value", value: "value", description: "The current value the loop is on.", },
    ], contentText: (parameters) => {
      return `Repeat from ${parameters["start"]} to ${parameters["end"]} by increments of ${parameters["step"]}`;
    },
  },
  {
    name: "Repeat while loop", value: "whileloop", description: "Executes the code inside the loop while the condition is true.", parameters: [
      { 
        name: "Condition", value: "condition", description: "The condition to check.", type: "condition", defaultValue: {
          type: "boolean",
          value: true, 
        },
      },
    ],
    codeInside: [
      { name: "Loop iteration", value: "loop", description: "The code to execute for each iteration.", },
    ],
    variables: [
      { name: "Iteration", value: "iteration", description: "The current iteration the loop is on.", },
    ], contentText: (parameters, parsers) => {
      return [`Repeat while`, ...parsers.parseCondition(parameters["condition"])];
    },
  },
  {
    name: "Notification", value: "notification", description: "Displays a notification.", parameters: [
      { name: "Title", value: "title", description: "The title of the notification.", type: "string", },
      { name: "Message", value: "message", description: "The message of the notification.", type: "string", },
    ], contentText: (parameters) => {
      return [`Send a notification with the title`, {type: "string", string: parameters.title}, `and content`, {type: "string", string: parameters.message}];
    },
  },
  {
    name: "Wait", value: "wait", description: "Waits for a certain amount of time.", parameters: [
      { name: "Time", value: "time", description: "The amount of time to wait, in miliseconds.", type: "number", defaultValue: 10, },
    ], contentText: (parameters, parsers) => {
      return [`Wait for`, parameters.time, `second${parameters.time === 1 ? "" : "s"}`];
    },
  },
  {
    name: "Set variable", value: "setvariable", description: "Sets a specified variable to a value.", parameters: [
      { name: "Variable", value: "variable", description: "The variable you want to set.", type: "string", defaultValue: "" },
      { name: "Value", value: "content", description: "The value you want to set the variable to.", type: "expression", defaultValue: {
          expression: { type: "arithmetic", left: { type: "number", value: 0 }, kind: "addition", right: { type: "number", value: 0 }, },
        },
      },
    ], contentText: (parameters, parsers) => {
      return [`Set the variable`, ...parsers.parseExpression({ type: "variable", variable: parameters.variable }), `to`, ...parsers.parseExpression(parameters.content)];
    },
  },
  {
    name: "Type string", value: "typestring", description: "Type a string", parameters: [
      { name: "String", value: "string", description: "The string you want to type.", type: "string", defaultValue: "", },
    ], contentText: (parameters) => {
      return [`Type the string`, {type: "string", string: parameters.string}];
    },
  },
  { name: "Stop", value: "stop", description: "Stops the macro.", parameters: [], contentText: (parameters) => {
      return ``;
    },
  },
  { name: "Move mouse relative", value: "movemouserelative", description: "Move the mouse relative to it's current position.", parameters: [
    { name: "X", value: "x", description: "How much to move the mouse horizontally", type: "number", defaultValue: 0, },
    { name: "Y", value: "y", description: "How much to move the mouse vertically", type: "number", defaultValue: 0, },
  ], contentText: (parameters) => {
    return `Move the mouse by (${parameters["x"]}, ${parameters["y"]})`;
  }},
  { name: "Move mouse absolute", value: "movemouseabsolute", description: "Move the mouse to a position on the screen.", parameters: [
    { name: "X", value: "x", description: "The horizontal position of the mouse", type: "number", defaultValue: 0, },
    { name: "Y", value: "y", description: "The vertical position of the mouse", type: "number", defaultValue: 0, },
  ], contentText: (parameters) => {
    return `Move the mouse to (${parameters["x"]}, ${parameters["y"]})`;
  }},
  { name: "Press key", value: "presskey", description: "Start pressing a key.", parameters: [
    { name: "Key", value: "key", description: "The key to press", type: "string", defaultValue: 0, }
  ], contentText: (parameters) => {
    return [`Start pressing the key`, {type: "string", string: parameters.key}];
  }},
  { name: "Release key", value: "releasekey", description: "Release a currently pressed key.", parameters: [
    { name: "Key", value: "key", description: "The key to press", type: "string", defaultValue: 0, }
  ], contentText: (parameters) => {
    return [`Release the key`, {type: "string", string: parameters.key}];
  }},
  { name: "Press mouse", value: "pressmouse", description: "Start pressing a mouse button.", parameters: [
    { name: "Button", value: "button", description: "The mouse button to press", type: "multiSelect", defaultValue: "LMB", options: [
      { name: "Left mouse button", value: "LMB", description: "The left mouse button"},
      { name: "Right mouse button", value: "RMB", description: "The right mouse button"},
    ] }
  ], contentText: (parameters) => {
    return [`Start pressing the mouse button`, {type: "string", string: parameters.button}];
  }},
  { name: "Release mouse", value: "releasemouse", description: "Release a currently pressed mouse button.", parameters: [
    { name: "Button", value: "button", description: "The mouse button to press", type: "multiSelect", defaultValue: "LMB", options: [
      { name: "Left mouse button", value: "LMB", description: "The left mouse button"},
      { name: "Right mouse button", value: "RMB", description: "The right mouse button"},
    ] }
  ], contentText: (parameters) => {
    return [`Release the mouse button`, {type: "string", string: parameters.button}];
  }},
  { name: "Read file", value: "readfile", description: "Read a file at a certain path.", parameters: [
    { name: "File", value: "file", description: "The file to read", type: "string", defaultValue: "C:/" },
    { name: "Variable", value: "variable", description: "The variable you want to set", type: "string", defaultValue: "fileContent" }
  ], contentText: (parameters) => {
    return [`Read the contents of the text file`, {type: "string", string: parameters.file}];
  }},
  { name: "Write to file", value: "writefile", description: "Write to an existing file.", parameters: [
    { name: "File", value: "file", description: "The file to write to", type: "string", defaultValue: "C:/" },
    { name: "Content", value: "data", description: "The content to write", type: "string", defaultValue: "Hello, world!" }
  ], contentText: (parameters) => {
    return [`Write`, {type: "string", string: parameters.content}, `to the text file`, {type: "string", string: parameters.file}];
  }}
];
