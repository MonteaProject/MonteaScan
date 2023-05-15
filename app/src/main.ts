import { Component } from '@angular/core';
import VlunsJson from '../../src/test/test.json';

interface VLUNS {
  id: Number;
  name: String;
  username: String;
  email: String;
}

const bodyName = document.querySelectorAll<Element>(".responsive-table__body__text--name");
const bodyStats = document.querySelectorAll<Element>(".responsive-table__body__text--status");
const bodyTypes = document.querySelectorAll<Element>(".responsive-table__body__text--types");
const bodyUpdate = document.querySelectorAll<Element>(".responsive-table__body__text--update");
const bodyCountry = document.querySelectorAll<Element>(".responsive-table__body__text--country");
const totalTableBodyRow = document.querySelectorAll<Element>(".responsive-table__body .responsive-table__row");

for (let i = 0; i < totalTableBodyRow.length; i++) {
  const headTitle = document.querySelector<HTMLElement>(".responsive-table__head__title--name");
  if (headTitle) {
    bodyName[i].setAttribute("data-title", headTitle.innerText);
  }

  const headStats = document.querySelector<HTMLElement>(".responsive-table__head__title--status");
  if (headStats) {
    bodyStats[i].setAttribute("data-title", headStats.innerText);
  }

  const headTypes = document.querySelector<HTMLElement>(".responsive-table__head__title--types");
  if (headTypes) {
    bodyTypes[i].setAttribute("data-title", headTypes.innerText);
  }

  const headUpdate = document.querySelector<HTMLElement>(".responsive-table__head__title--update");
  if (headUpdate) {
    bodyUpdate[i].setAttribute("data-title", headUpdate.innerText);
  }

  const headCountry = document.querySelector<HTMLElement>(".responsive-table__head__title--country");
  if (headCountry) {
    bodyCountry[i].setAttribute("data-title", headCountry.innerText);
  }
}

@Component({
  selector: 'app-root',
  templateUrl: './index.html',
  styleUrls: ['./styles.scss']
})

export class AppComponent {
  vluns: VLUNS[] = VlunsJson;

  constructor() {
    console.log(this.vluns);
  }
}
