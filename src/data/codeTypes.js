export default [
  {
    name: "If", value: "if", description: "Executes the code inside if the condition is true.", parameters: [
      {
        name: "Condition", value: "condition", description: "The condition to check.", type: "condition", defaultValue: {
          condition: { type: "boolean", value: true, },
        },
      },
    ],
    codeInside: [
      { name: "Then", value: "then", description: "The code to execute if the condition is true.", },
      { name: "Else", value: "else", description: "The code to execute if the condition is false.", },
    ], contentText: (parameters) => {
      return `If ${parameters["condition"]}`;
    },
  },
  {
    name: "Function call", value: "function", description: "Executes the code inside the function it calls.", parameters: [
      { name: "Function", value: "function", type: "function", description: "The function to call.", },
    ], contentText: (parameters) => {
      return `Call the function "${parameters["function"]}"`;
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
          condition: { type: "boolean", value: true }, 
        },
      },
    ],
    codeInside: [
      { name: "Loop iteration", value: "loop", description: "The code to execute for each iteration.", },
    ],
    variables: [
      { name: "Iteration", value: "iteration", description: "The current iteration the loop is on.", },
    ], contentText: (parameters) => {
      return `Repeat while ${parameters["condition"]}`;
    },
  },
  {
    name: "Notification", value: "notification", description: "Displays a notification.", parameters: [
      { name: "Title", value: "title", description: "The title of the notification.", type: "string", },
      { name: "Message", value: "message", description: "The message of the notification.", type: "string", },
    ], contentText: (parameters) => {
      return `Send a notification with the title "${parameters.title}" and content "${parameters.message}"`;
    },
  },
  {
    name: "Wait", value: "wait", description: "Waits for a certain amount of time.", parameters: [
      { name: "Time", value: "time", description: "The amount of time to wait, in miliseconds.", type: "number", defaultValue: 10, },
    ], contentText: (parameters) => {
      return `Wait for "${parameters.time}" milliseconds`;
    },
  },
  {
    name: "Set variable", value: "setvariable", description: "Sets a specified variable to a value.", parameters: [
      { name: "Variable", value: "variable", description: "The variable you want to set.", type: "string", defaultValue: "" },
      { name: "Value", value: "content", description: "The value you want to set the variable to.", type: "expression", defaultValue: {
          expression: { type: "arithmetic", left: { type: "number", value: 0 }, kind: "addition", right: { type: "number", value: 0 }, },
        },
      },
    ], contentText: (parameters) => {
      return `Set the variable "${parameters.variable}" to "${parameters.content}"`;
    },
  },
  {
    name: "Type string", value: "typestring", description: "Type a string", parameters: [
      { name: "String", value: "string", description: "The string you want to type.", type: "string", defaultValue: "", },
    ], contentText: (parameters) => {
      return `Type the string "${parameters.string}"`;
    },
  },
  { name: "Stop", value: "stop", description: "Stops the macro.", parameters: [], contentText: (parameters) => {
      return ``;
    },
  },
];