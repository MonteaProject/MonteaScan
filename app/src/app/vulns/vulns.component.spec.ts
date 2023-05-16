import { ComponentFixture, TestBed } from '@angular/core/testing';

import { VulnsComponent } from './vulns.component';

describe('VulnsComponent', () => {
  let component: VulnsComponent;
  let fixture: ComponentFixture<VulnsComponent>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      declarations: [VulnsComponent]
    });
    fixture = TestBed.createComponent(VulnsComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
