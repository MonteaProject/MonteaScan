import { Component } from '@angular/core';
import VlunsJson from '../../../src/test/test.json';

interface Vluns {
  id: Number;
  name: String;
  username: String;
  email: String;
}

@Component({
  selector: 'app-root',
  templateUrl: './app.component.html',
  styleUrls: ['./app.component.scss']
})
export class AppComponent {
  vluns: Vluns[] = VlunsJson;

  constructor() {
    console.log(this.vluns);
  }
  title = 'Result Vluns';
}
