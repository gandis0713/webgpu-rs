import('./pkg').then(md => {
  console.log("md : ", md);
  md.create();
})
  .catch(console.error);