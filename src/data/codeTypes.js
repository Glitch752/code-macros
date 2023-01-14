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
      return [`If`, ...parsers.parseCondition(parameters.condition)];
    },
  },
  {
    name: "Function call", value: "function", description: "Executes the code inside the function it calls.", parameters: [
      { name: "Function", value: "function", type: "function", description: "The function to call." },
    ], contentText: (parameters) => {
      return [`Call the function`, {type: "string", string: parameters.function}];
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
      return `Repeat from ${parameters.start} to ${parameters.end} by increments of ${parameters.step}`;
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
      return [`Repeat while`, ...parsers.parseCondition(parameters.condition)];
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
    ], contentText: (parameters) => {
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
  { name: "Stop", value: "stop", description: "Stops the macro.", parameters: [], contentText: () => {
      return ``;
    },
  },
  { name: "Move mouse relative", value: "movemouserelative", description: "Move the mouse relative to it's current position.", parameters: [
    { name: "X", value: "x", description: "How much to move the mouse horizontally", type: "number", defaultValue: 0, },
    { name: "Y", value: "y", description: "How much to move the mouse vertically", type: "number", defaultValue: 0, },
  ], contentText: (parameters) => {
    return `Move the mouse by (${parameters.x}, ${parameters.y})`;
  }},
  { name: "Move mouse absolute", value: "movemouseabsolute", description: "Move the mouse to a position on the screen.", parameters: [
    { name: "X", value: "x", description: "The horizontal position of the mouse", type: "number", defaultValue: 0, },
    { name: "Y", value: "y", description: "The vertical position of the mouse", type: "number", defaultValue: 0, },
  ], contentText: (parameters) => {
    return `Move the mouse to (${parameters.x}, ${parameters.y})`;
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
    return [`Read the contents of the text file`, {type: "string", string: parameters.file}, `and set the variable`, {type: "variable", variable: parameters.variable}];
  }},
  { name: "Write to file", value: "writefile", description: "Write to a file. Creates a new file if none exists.", parameters: [
    { name: "File", value: "file", description: "The file to write to", type: "string", defaultValue: "C:/" },
    { name: "Content", value: "data", description: "The content to write", type: "string", defaultValue: "Hello, world!" }
  ], contentText: (parameters) => {
    return [`Write`, {type: "string", string: parameters.data}, `to the text file`, {type: "string", string: parameters.file}];
  }},
  { name: "Delete file", value: "deletefile", description: "Delete an existing file.", parameters: [
    { name: "File", value: "file", description: "The file to delete", type: "string", defaultValue: "C:/" }
  ], contentText: (parameters) => {
    return [`Delete the file`, {type: "string", string: parameters.file}];
  }},
  { name: "Create folder", value: "createfolder", description: "Create a folder at a certain path.", parameters: [
    { name: "Path", value: "path", description: "The path to make a folder at", type: "string", defaultValue: "C:/" }
  ], contentText: (parameters) => {
    return [`Create a folder at`, {type: "string", string: parameters.path}];
  }},
  { name: "Delete folder", value: "deletefolder", description: "Delete an existing folder recursively.", parameters: [
    { name: "Path", value: "path", description: "The path to delete", type: "string", defaultValue: "C:/" }
  ], contentText: (parameters) => {
    return [`Delete the folder at`, {type: "string", string: parameters.path}];
  }},
  { name: "Get variable data type", value: "getdatatype", description: "Get the data type of a variable.", parameters: [
    { name: "Variable", value: "variable", description: "The variable to get the data type of", type: "string", defaultValue: "myVariable" },
    { name: "Output", value: "output", description: "The variable to set to the data type", type: "string", defaultValue: "myVariableType" }
  ], contentText: (parameters) => {
    return [`Get the data type of the variable`, {type: "variable", variable: parameters.variable}, `and set the variable`, {type: "variable", variable: parameters.output}];
  }},
  { name: "Create empty array", value: "createarray", description: "Create an empty array.", parameters: [
    { name: "Variable", value: "variable", description: "The variable to set as the array", type: "string", defaultValue: "myArray" }
  ], contentText: (parameters) => {
    return [`Create a new empty array and set the variable`, { type: "variable", variable: parameters.variable }];
  }},
  { name: "Add item to array", value: "addtoarray", description: "Appends a variable to the end of an array by value.", parameters: [
    { name: "Array", value: "array", description: "The array to add an item to", type: "string", defaultValue: "myArray" },
    { name: "Item", value: "data", description: "The item to add, which comes from a variable.", type: "variable", defaultValue: 0 }
  ], contentText: (parameters) => {
    return [`Add`, {type: "variable", variable: parameters.data}, `to the array`, { type: "variable", variable: parameters.array }];
  }},
  { name: "Remove item from array", value: "removefromarray", description: "Removes the item at a certain index from the array. Arrays are 0-indexed.", parameters: [
    { name: "Array", value: "array", description: "The array to remove an item from", type: "string", defaultValue: "myArray" },
    { name: "Index", value: "index", description: "The index of the item to remove", type: "expression", defaultValue: {
      expression: { type: "arithmetic", left: { type: "number", value: 0 }, kind: "addition", right: { type: "number", value: 0 }, },
    } }
  ], contentText: (parameters) => {
    return [`Remove the item at index`, {type: "variable", variable: parameters.index}, `from the array`, { type: "variable", variable: parameters.array }];
  }},
  { name: "Get array length", value: "getarraylength", description: "Get the length of an array.", parameters: [
    { name: "Array", value: "array", description: "The array to get the length of", type: "string", defaultValue: "myArray" },
    { name: "Output", value: "output", description: "The variable to set to the length", type: "string", defaultValue: "myArrayLength" }
  ], contentText: (parameters) => {
    return [`Get the length of the array`, { type: "variable", variable: parameters.array }, `and set the variable`, { type: "variable", variable: parameters.output }];
  }},
  { name: "Loop through array", value: "looparray", description: "Loop through an array and execute the actions inside the loop for each item.", parameters: [
    { name: "Array", value: "array", description: "The array to loop through", type: "string", defaultValue: "myArray" },
  ],
  codeInside: [
    { name: "Loop iteration", value: "loop", description: "The code to execute for each iteration.", },
  ],
  variables: [
    { name: "Item", value: "item", description: "The item at the current index of the array.", },
  ], contentText: (parameters) => {
    return `Repeat from ${parameters.start} to ${parameters.end} by increments of ${parameters.step}`;
  }},
  { name: "Get item from array", value: "getarrayindex", description: "Get the item at a certain index from the array. Arrays are 0-indexed.", parameters: [
    { name: "Array", value: "array", description: "The array to get an item from", type: "string", defaultValue: "myArray" },
    { name: "Index", value: "index", description: "The index of the item to get", type: "expression", defaultValue: {
      expression: { type: "arithmetic", left: { type: "number", value: 0 }, kind: "addition", right: { type: "number", value: 0 }, },
    } },
    { name: "Output", value: "output", description: "The variable to set to the item", type: "string", defaultValue: "myArrayItem" }
  ], contentText: (parameters) => {
    return [`Get the item at index`, {type: "variable", variable: parameters.index}, `from the array`, { type: "variable", variable: parameters.array }, `and set the variable`, { type: "variable", variable: parameters.output }];
  }},
  { name: "Set item in array", value: "setarrayindex", description: "Set the item at a certain index in the array. Arrays are 0-indexed.", parameters: [
    { name: "Array", value: "array", description: "The array to set an item in", type: "string", defaultValue: "myArray" },
    { name: "Index", value: "index", description: "The index of the item to set", type: "expression", defaultValue: {
      expression: { type: "arithmetic", left: { type: "number", value: 0 }, kind: "addition", right: { type: "number", value: 0 }, },
    } },
    { name: "Item", value: "data", description: "The item to set, which comes from a variable.", type: "variable", defaultValue: 0 }
  ], contentText: (parameters) => {
    return [`Set the item at index`, {type: "variable", variable: parameters.index}, `in the array`, { type: "variable", variable: parameters.array }, `to`, { type: "variable", variable: parameters.data }];
  }},
  { name: "Get folder contents", value: "getfoldercontents", description: "Get the contents of a folder. Returns an array of paths.", parameters: [
    { name: "Folder", value: "folder", description: "The folder to get the contents of", type: "string", defaultValue: "C:/myFolder" },
    { name: "Output", value: "output", description: "The variable to set to the contents", type: "string", defaultValue: "myFolderContents" }
  ], contentText: (parameters) => {
    return [`Get the contents of the folder`, { type: "variable", variable: parameters.folder }, `and set the variable`, { type: "variable", variable: parameters.output }];
  }},
  { name: "Log", value: "log", description: "Add a message to the log.", parameters: [
    { name: "Message", value: "message", description: "The message to log", type: "string", defaultValue: "Hello world!" },
  ], contentText: (parameters) => {
    return [`Log the message`, { type: "string", string: parameters.message }];
  }},
  { name: "Clear log", value: "clearlog", description: "Clear the log.", parameters: [], contentText: () => {
    return ``;
  }},
  { name: "Split string", value: "splitstring", description: "Split a string into an array of strings.", parameters: [
    { name: "String", value: "string", description: "The string to split", type: "string", defaultValue: "Hello world!" },
    { name: "Splitter", value: "splitter", description: "The string to split the string by", type: "string", defaultValue: " " },
    { name: "Output", value: "output", description: "The variable to set to the array of strings", type: "string", defaultValue: "myStringArray" }
  ], contentText: (parameters) => {
    return [`Split the string`, { type: "string", string: parameters.string }, `by the string`, { type: "string", string: parameters.splitter }, `and set the variable`, { type: "variable", variable: parameters.output }];
  }},
  { name: "Join strings", value: "joinstrings", description: "Join an array of strings into a single string.", parameters: [
    { name: "Array", value: "array", description: "The array of strings to join", type: "string", defaultValue: "myStringArray" },
    { name: "Joiner", value: "joiner", description: "The string to join the strings by", type: "string", defaultValue: ", " },
    { name: "Output", value: "output", description: "The variable to set to the joined string", type: "string", defaultValue: "myJoinedString" }
  ], contentText: (parameters) => {
    return [`Join the strings in the array`, { type: "variable", variable: parameters.array }, `by the string`, { type: "string", string: parameters.joiner }, `and set the variable`, { type: "variable", variable: parameters.output }];
  }},
  { name: "Reverse array", value: "reversearray", description: "Reverse an array.", parameters: [
    { name: "Array", value: "array", description: "The array to reverse", type: "string", defaultValue: "myArray" },
    { name: "Output", value: "output", description: "The variable to set to the reversed array", type: "string", defaultValue: "myReversedArray" }
  ], contentText: (parameters) => {
    return [`Reverse the array`, { type: "variable", variable: parameters.array }, `and set the variable`, { type: "variable", variable: parameters.output }];
  }},
  { name: "Sort array", value: "sortarray", description: "Sort an array.", parameters: [
    { name: "Array", value: "array", description: "The array to sort", type: "string", defaultValue: "myArray" },
    { name: "Output", value: "output", description: "The variable to set to the sorted array", type: "string", defaultValue: "mySortedArray" }
  ], contentText: (parameters) => {
    return [`Sort the array`, { type: "variable", variable: parameters.array }, `and set the variable`, { type: "variable", variable: parameters.output }];
  }},
];